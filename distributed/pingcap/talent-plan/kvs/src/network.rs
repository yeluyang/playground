extern crate serde;
use serde::{Deserialize, Serialize};

pub extern crate serde_json as protocol_serde;

/// Protocol
#[derive(Debug, Deserialize, Serialize)]
pub enum Protocol {
    GetRequest(String),
    GetResponse(Option<String>),

    SetRequest { key: String, value: String },
    SetResponse(()),

    RemoveRequest(String),
    RemoveResponse(()),

    Error(String),
}

impl Protocol {
    pub fn to_bytes(&self) -> Vec<u8> {
        protocol_serde::to_vec(self).unwrap_or_else(|err| {
            protocol_serde::to_vec(&Protocol::Error(err.to_string())).unwrap()
        })
    }
}

impl<'a> From<&'a str> for Protocol {
    fn from(s: &'a str) -> Self {
        protocol_serde::from_str(s).unwrap_or_else(|err| Protocol::Error(err.to_string()))
    }
}

impl<'a> From<&'a [u8]> for Protocol {
    fn from(b: &'a [u8]) -> Self {
        protocol_serde::from_slice(b).unwrap()
    }
}
