use std::{collections::HashMap, ops::RangeInclusive};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json as serde_fmt;

pub trait Record: From<Vec<u8>> {
    fn to_bytes(&self) -> Vec<u8>;
    fn key(&self) -> String;
}

pub type LogEntryKey = String;
pub type LogEntryIndex = HashMap<LogEntryKey, usize>;

#[derive(Debug, Eq, Clone, Serialize, Deserialize)]
pub struct LogFileHeader {
    pub version: usize,
    pub ids: RangeInclusive<usize>,
    pub compacted: bool,
}

impl PartialEq for LogFileHeader {
    fn eq(&self, other: &Self) -> bool {
        self.ids == other.ids && self.compacted == other.compacted
    }
}

impl Default for LogFileHeader {
    fn default() -> Self {
        Self::new(RangeInclusive::new(0, 0), false)
    }
}

impl LogFileHeader {
    pub fn new(ids: RangeInclusive<usize>, compacted: bool) -> Self {
        LogFileHeader {
            version: 0,
            ids,
            compacted,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogEntryData {
    pub key: LogEntryKey,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum LogEntry {
    FileHeader(LogFileHeader),
    Index(usize, LogEntryIndex),
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
            key: r.key(),
            data: r.to_bytes(),
        })
    }
}

#[derive(Debug)]
pub struct LogEntryPointer {
    pub file_id: usize,
    pub key: LogEntryKey,
}

impl LogEntryPointer {
    pub fn new(file_id: usize, key: String) -> Self {
        LogEntryPointer { file_id, key }
    }
}
