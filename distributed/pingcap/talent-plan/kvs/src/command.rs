extern crate serde;
use serde::{Deserialize, Serialize};

pub extern crate serde_json as command_serde;

extern crate lsmt;
use lsmt::Record;

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Record for Command {
    fn to_bytes(&self) -> Vec<u8> {
        command_serde::to_vec(self).unwrap_or_else(|err| {
            panic!(
                "failed to ser Command to bytes: err={}, value={:?}",
                err, self
            )
        })
    }
    fn key(&self) -> String {
        match self {
            Command::Set { key, .. } => key.clone(),
            Command::Remove { key } => key.clone(),
        }
    }
}

impl From<&[u8]> for Command {
    fn from(data: &[u8]) -> Self {
        command_serde::from_slice(data).unwrap_or_else(|err| {
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
