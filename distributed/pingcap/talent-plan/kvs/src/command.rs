extern crate serde;
extern crate serde_json as serde_format;
use serde::{Deserialize, Serialize};

extern crate lsmt;
use lsmt::Record;

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Record for Command {
    fn to_bytes(&self) -> Vec<u8> {
        serde_format::to_vec(self).unwrap_or_else(|err| {
            panic!(
                "failed to ser Command to bytes: err={}, value={:?}",
                err, self
            )
        })
    }
}

impl From<&[u8]> for Command {
    fn from(data: &[u8]) -> Self {
        serde_format::from_slice(data).unwrap_or_else(|err| {
            panic!(
                "failed to de Command from bytes: err={}, value={:?}",
                err, data
            )
        })
    }
}

impl From<Vec<u8>> for Command {
    fn from(data: Vec<u8>) -> Self {
        Self::from(data.as_slice())
    }
}
