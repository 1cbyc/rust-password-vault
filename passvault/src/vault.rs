use crate::entry::ServiceInfo;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng, generic_array::GenericArray};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;
use serde::{Serialize, Deserialize};
use base64::{encode, decode};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const PBKDF2_ITER: u32 = 100_000;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    entries: Vec<ServiceInfo>,
    salt: String,
}

pub enum VaultError {
    Io,
    Crypto,
    Serde,
}

impl Vault {
    pub fn new(master: &str) -> Self {
        let mut salt = [0u8; SALT_LEN];
        OsRng.fill_bytes(&mut salt);
        Self { entries: vec![], salt: encode(salt) }
    }
    pub fn open(path: &str, master: &str) -> Result<Self, VaultError> {
        let mut f = File::open(path).map_err(|_| VaultError::Io)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).map_err(|_| VaultError::Io)?;
        let (salt, nonce, ct) = Self::split(&buf)?;
        let key = Self::derive_key(master, &salt);
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let pt = cipher.decrypt(Nonce::from_slice(&nonce), ct.as_ref()).map_err(|_| VaultError::Crypto)?;
        let mut vault: Vault = serde_json::from_slice(&pt).map_err(|_| VaultError::Serde)?;
        vault.salt = encode(salt);
        Ok(vault)
    }
    pub fn save(&self, path: &str, master: &str) -> Result<(), VaultError> {
        let salt = decode(&self.salt).map_err(|_| VaultError::Crypto)?;
        let key = Self::derive_key(master, &salt);
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let mut nonce = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut nonce);
        let pt = serde_json::to_vec(self).map_err(|_| VaultError::Serde)?;
        let ct = cipher.encrypt(Nonce::from_slice(&nonce), pt.as_ref()).map_err(|_| VaultError::Crypto)?;
        let mut f = OpenOptions::new().write(true).create(true).truncate(true).open(path).map_err(|_| VaultError::Io)?;
        f.write_all(&salt).map_err(|_| VaultError::Io)?;
        f.write_all(&nonce).map_err(|_| VaultError::Io)?;
        f.write_all(&ct).map_err(|_| VaultError::Io)?;
        Ok(())
    }
    pub fn add(&mut self, entry: ServiceInfo) {
        self.entries.push(entry);
    }
    pub fn entries(&self) -> &Vec<ServiceInfo> {
        &self.entries
    }
    pub fn delete(&mut self, service: &str) {
        self.entries.retain(|e| e.service != service);
    }
    fn derive_key(master: &str, salt: &[u8]) -> [u8; 32] {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(master.as_bytes(), salt, PBKDF2_ITER, &mut key);
        key
    }
    fn split(buf: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), VaultError> {
        if buf.len() < SALT_LEN + NONCE_LEN {
            return Err(VaultError::Crypto);
        }
        let salt = buf[..SALT_LEN].to_vec();
        let nonce = buf[SALT_LEN..SALT_LEN+NONCE_LEN].to_vec();
        let ct = buf[SALT_LEN+NONCE_LEN..].to_vec();
        Ok((salt, nonce, ct))
    }
} 