use crate::entry::ServiceInfo;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use serde_json;
use csv::{ReaderBuilder, WriterBuilder};
pub fn export_json(path: &str, entries: &[ServiceInfo]) {
    let f = File::create(path).unwrap();
    serde_json::to_writer_pretty(f, entries).unwrap();
}
pub fn import_json(path: &str) -> Vec<ServiceInfo> {
    let f = File::open(path).unwrap();
    serde_json::from_reader(f).unwrap()
}
pub fn export_csv(path: &str, entries: &[ServiceInfo]) {
    let f = File::create(path).unwrap();
    let mut wtr = WriterBuilder::new().from_writer(f);
    for e in entries {
        wtr.serialize(e).unwrap();
    }
    wtr.flush().unwrap();
}
pub fn import_csv(path: &str) -> Vec<ServiceInfo> {
    let f = File::open(path).unwrap();
    let mut rdr = ReaderBuilder::new().from_reader(f);
    rdr.deserialize().map(|r| r.unwrap()).collect()
} 