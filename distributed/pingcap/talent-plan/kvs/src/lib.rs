#![deny(missing_docs)]

//! kvs

use std::{collections::HashMap, fs, path::Path};

extern crate lsmt;
use lsmt::{LogEntryPointer, LogStructuredMergeTree};

mod errors;
pub use errors::{Error, Result};

mod command;
use command::Command;

/// KvStore
pub struct KvStore {
    data: HashMap<String, LogEntryPointer>,
    wal: LogStructuredMergeTree,
}

impl KvStore {
    /// open an object of KvStore on an exist log file and return it
    ///
    /// # Arguments
    ///
    /// - dir: path to persistent dir
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<KvStore> {
        if !dir.as_ref().exists() {
            fs::create_dir(&dir)?;
        }

        let mut kvs_store = KvStore {
            data: HashMap::default(),
            wal: LogStructuredMergeTree::open(lsmt::Config {
                lsmt_dir: dir
                    .as_ref()
                    .to_str()
                    .map(|s| s.to_owned())
                    .ok_or(Error::InvalidPath(dir.as_ref().as_os_str().to_os_string()))?,
                file_size: 1000,
                compact_enable: true,
                merge_threshold: Some(2),
            })?,
        };

        while let Some(p) = kvs_store.wal.pop()? {
            kvs_store.data.insert(p.key.clone(), p);
        }

        Ok(kvs_store)
    }

    /// insert a key-value into KvStore
    ///
    /// # Arguments
    /// - key: String type
    /// - value: String type
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set { key, value };
        let p = self.wal.append(&cmd)?;
        self.data.insert(p.key.clone(), p);
        Ok(())
    }

    /// get the value of key
    ///
    /// # Arguments
    /// - key: String type
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(p) = self.data.get(&key) {
            if let Some(c) = self.wal.read_by_pointer::<Command>(p)? {
                match c {
                    Command::Set { value, .. } => Ok(Some(value)),
                    Command::Remove { .. } => Ok(None),
                }
            } else {
                Err(Error::DataNotFound(key))
            }
        } else {
            Ok(None)
        }
    }

    /// remove a key-value from object of KvStore
    ///
    /// # Arguments
    /// - key: String type
    pub fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove { key: key.clone() };
        self.wal.append(&cmd)?;
        self.data
            .remove(&key)
            .ok_or(Error::KeyNotFound(key.clone()))?;
        Ok(())
    }
}
