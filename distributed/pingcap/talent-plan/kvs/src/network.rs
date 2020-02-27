use std::{
    io::{BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

extern crate serde;
use serde::{Deserialize, Serialize};

pub extern crate serde_json as protocol_serde;

use crate::errors::{Error, Result};

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

pub struct Client {
    net_reader: BufReader<TcpStream>,
    net_writer: BufWriter<TcpStream>,
}

impl Client {
    pub fn connect(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Client {
            net_reader: BufReader::new(stream.try_clone()?),
            net_writer: BufWriter::new(stream),
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        protocol_serde::to_writer(&mut self.net_writer, &Protocol::GetRequest(key))?;
        self.net_writer.flush()?;

        let mut buf: Vec<u8> = Vec::new();
        self.net_reader.read_to_end(&mut buf)?;
        let p = Protocol::from(buf.as_slice());
        match p {
            Protocol::GetResponse(opt) => Ok(opt),
            Protocol::Error(err) => Err(Error::Simple(err)),
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        protocol_serde::to_writer(&mut self.net_writer, &Protocol::SetRequest { key, value })?;
        self.net_writer.flush()?;

        let mut buf: Vec<u8> = Vec::new();
        self.net_reader.read_to_end(&mut buf)?;
        let p = Protocol::from(buf.as_slice());
        match p {
            Protocol::SetResponse(_) => Ok(()),
            Protocol::Error(err) => Err(Error::Simple(err)),
            _ => unreachable!(),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        protocol_serde::to_writer(&mut self.net_writer, &Protocol::RemoveRequest(key))?;
        self.net_writer.flush()?;

        let mut buf: Vec<u8> = Vec::new();
        self.net_reader.read_to_end(&mut buf)?;
        let p = Protocol::from(buf.as_slice());
        match p {
            Protocol::RemoveResponse(_) => Ok(()),
            Protocol::Error(err) => Err(Error::Simple(err)),
            _ => unreachable!(),
        }
    }
}
