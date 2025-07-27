use libreauth::oath::TOTPBuilder;
pub fn generate_totp(secret: &str) -> String {
    let totp = TOTPBuilder::new()
        .base32_key(secret)
        .finalize()
        .unwrap();
    totp.generate_current().unwrap()
} 