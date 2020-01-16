#![deny(missing_docs)]

//! kvs

use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::Path,
};

extern crate lsmt;
use lsmt::LogStructuredMergeTree;

mod errors;
pub use errors::{Error, Result};

mod command;
use command::Command;

/// KvStore
pub struct KvStore {
    seq_id: usize,
    data: HashMap<String, String>,
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

        let persistent_path = dir.as_ref().join("0.wal");
        let fd = if persistent_path.exists() {
            OpenOptions::new()
                .append(true)
                .read(true)
                .open(persistent_path)?
        } else {
            OpenOptions::new()
                .append(true)
                .read(true)
                .create(true)
                .open(persistent_path)?
        };

        let mut kvs_store = KvStore {
            seq_id: 0,
            data: HashMap::default(),
            wal: LogStructuredMergeTree::new(fd)?,
        };

        while let Some(data) = kvs_store.wal.read_next()? {
            let cmd = Command::from(data.as_slice());
            kvs_store.play(&cmd)?;
        }

        Ok(kvs_store)
    }

    /// insert a key-value into KvStore
    ///
    /// # Arguments
    /// - key: String type
    /// - value: String type
    ///
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let key = String::from("key");
    /// let value = String::from("value");
    /// let mut kvs = KvStore::new();
    /// kvs.set(key.clone(), value.clone());
    /// assert_eq!(kvs.get(key).unwrap(), value);
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set { key, value };
        self.play(&cmd)?;
        self.wal.append(cmd.as_bytes().as_slice())?;
        Ok(())
    }

    /// get the value of key
    ///
    /// # Arguments
    /// - key: String type
    ///
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let key = String::from("key");
    /// let value = String::from("value");
    ///
    /// let mut kvs = KvStore::new();
    /// assert_eq!(kvs.get(key.to_owned()), None);
    ///
    /// kvs.set(key.clone(), value.clone());
    /// assert_eq!(kvs.get(key).unwrap(), value);
    /// ```
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.data
            .get(&key)
            .map(|val| Some(val.to_owned()))
            .ok_or(Error::KeyNotFound(key))
    }

    /// remove a key-value from object of KvStore
    ///
    /// # Arguments
    /// - key: String type
    ///
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let key = String::from("key");
    /// let mut kvs = KvStore::new();
    /// kvs.remove(key);
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove { key };
        self.play(&cmd)?;
        self.wal.append(cmd.as_bytes().as_slice())?;
        Ok(())
    }

    fn play(&mut self, cmd: &Command) -> Result<()> {
        match cmd {
            Command::Set { key, value } => {
                self.data.insert(key.to_owned(), value.to_owned());
                Ok(())
            }
            Command::Remove { key } => {
                let key = key.to_owned();
                self.data
                    .remove(&key)
                    .ok_or(Error::KeyNotFound(key))
                    .map(|_| ())
            }
        }
        .map(|_| {
            self.seq_id += 1;
        })
    }
}
