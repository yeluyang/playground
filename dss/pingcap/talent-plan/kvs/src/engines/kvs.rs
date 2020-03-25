use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

extern crate lsmt;
use lsmt::{LogEntryPointer, LogStructuredMergeTree};

use crate::command::Command;
use crate::errors::{Error, Result};

struct KvStoreInner {
    data: HashMap<String, LogEntryPointer>,
    wal: LogStructuredMergeTree,
}

impl KvStoreInner {
    /// open an object of KvStoreInner on an exist log file and return it
    ///
    /// # Arguments
    ///
    /// - dir: path to persistent dir
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<Self> {
        debug!("open kv storage on directory={:?}", dir.as_ref());

        if !dir.as_ref().exists() {
            fs::create_dir(&dir)?;
        }

        trace!("constructing kv storage");
        let mut kvs_store = Self {
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

        trace!("loading data from wal");
        while let Some(p) = kvs_store.wal.pop()? {
            kvs_store.data.insert(p.key.clone(), p);
        }

        Ok(kvs_store)
    }

    /// insert a key-value into KvStoreInner
    ///
    /// # Arguments
    /// - key: String type
    /// - value: String type
    fn set(&mut self, key: String, value: String) -> Result<()> {
        debug!("setting data={{key={}, val={}}}", key, value);

        let cmd = Command::Set { key, value };

        trace!("writting data in wal");
        let p = self.wal.append(&cmd)?;

        trace!("inserting data in memory table");
        self.data.insert(p.key.clone(), p);

        Ok(())
    }

    /// get the value of key
    ///
    /// # Arguments
    /// - key: String type
    fn get(&mut self, key: String) -> Result<Option<String>> {
        debug!("getting data by key={}", key);

        if let Some(p) = self.data.get(&key) {
            trace!("get pointer from wal: pointer={:?}", p);
            if let Some(c) = self.wal.read_by_pointer::<Command>(p)? {
                trace!("command read by pointer from wal: command={:?}", c);
                match c {
                    Command::Set { value, .. } => Ok(Some(value)),
                    Command::Remove { .. } => Ok(None),
                }
            } else {
                // XXX: should be internal error?
                Err(Error::DataNotFound(key))
            }
        } else {
            trace!("data of key not found");
            Ok(None)
        }
    }

    /// remove a key-value from object of KvStoreInner
    ///
    /// # Arguments
    /// - key: String type
    fn remove(&mut self, key: String) -> Result<()> {
        debug!("removing data by key={}", key);

        let cmd = Command::Remove { key: key.clone() };

        trace!("removing data from memory table");
        if self.data.remove(&key).is_some() {
            trace!("writting tombstone in wal");
            self.wal.append(&cmd)?;

            Ok(())
        } else {
            Err(Error::KeyNotFound(key.clone()))
        }
    }
}

/// KvStore
#[derive(Clone)]
pub struct KvStore {
    inner: Arc<Mutex<KvStoreInner>>,
}

impl KvStore {
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(KvStoreInner::open(dir)?)),
        })
    }
}

impl super::KvsEngine for KvStore {
    fn set(&self, key: String, value: String) -> Result<()> {
        self.inner.lock().unwrap().set(key, value)
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        self.inner.lock().unwrap().get(key)
    }

    fn remove(&self, key: String) -> Result<()> {
        self.inner.lock().unwrap().remove(key)
    }
}
