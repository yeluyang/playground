extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json as serde_fmt;

pub trait Record: From<Vec<u8>> {
    fn to_bytes(&self) -> Vec<u8>;
    fn get_entry_key(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    key: String,
    pub data: Vec<u8>,
}

impl LogEntry {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_fmt::to_vec(self).unwrap()
    }
}

impl From<Vec<u8>> for LogEntry {
    fn from(b: Vec<u8>) -> Self {
        serde_fmt::from_slice(b.as_slice()).unwrap()
    }
}

impl<T: Record> From<&T> for LogEntry {
    fn from(r: &T) -> Self {
        LogEntry {
            key: r.get_entry_key(),
            data: r.to_bytes(),
        }
    }
}
