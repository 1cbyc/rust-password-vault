use reqwest::blocking::Client;
use std::fs::File;
use std::io::{Read, Write};
pub fn upload(url: &str, path: &str) {
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let client = Client::new();
    let _ = client.post(url).body(buf).send().unwrap();
}
pub fn download(url: &str, path: &str) {
    let client = Client::new();
    let mut resp = client.get(url).send().unwrap();
    let mut f = File::create(path).unwrap();
    let mut buf = Vec::new();
    resp.copy_to(&mut buf).unwrap();
    f.write_all(&buf).unwrap();
} 