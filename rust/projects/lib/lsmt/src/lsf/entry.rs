use std::{collections::HashMap, ops::RangeInclusive};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json as serde_fmt;

pub trait Record: From<Vec<u8>> {
    fn to_bytes(&self) -> Vec<u8>;
    fn get_entry_key(&self) -> String;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogFileHeader {
    pub version: usize,
    pub ids: RangeInclusive<usize>,
    pub compacted: bool,
    pub entry_count: usize,
}

impl Default for LogFileHeader {
    fn default() -> Self {
        Self::new(RangeInclusive::new(0, 0), false, 0)
    }
}

impl LogFileHeader {
    pub fn new(ids: RangeInclusive<usize>, compacted: bool, entry_count: usize) -> Self {
        LogFileHeader {
            version: 0,
            ids,
            compacted,
            entry_count,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogEntryData {
    key: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum LogEntry {
    FileHeader(LogFileHeader),
    Index(HashMap<String, usize>),
    Data(LogEntryData),
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
        LogEntry::Data(LogEntryData {
            key: r.get_entry_key(),
            data: r.to_bytes(),
        })
    }
}

pub struct LogEntryPointer {
    pub file_id: usize,
    pub entry_key: String,
}
