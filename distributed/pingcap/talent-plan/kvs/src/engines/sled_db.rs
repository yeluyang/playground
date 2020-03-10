use std::path::Path;

extern crate sled;
use sled::Db;

use crate::errors::{Error, Result};

pub struct SledKvsEngine {
    inner: Db,
}

impl SledKvsEngine {
    /// new
    pub fn new(db: sled::Db) -> Self {
        Self { inner: db }
    }
    /// open
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<Self> {
        debug!("open sled storage on directory={:?}", dir.as_ref());
        Ok(Self {
            inner: sled::Db::start_default(dir)?,
        })
    }
}

impl super::KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        debug!("setting data={{key={}, value={}}}", key, value);

        self.inner.set(key, value.as_str())?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        debug!("getting data by key={}", key);

        if let Some(val) = self.inner.get(&key)? {
            let val = String::from_utf8(val.to_vec())?;
            trace!("found data={{key={}, val={}}}", key, val);
            Ok(Some(val))
        } else {
            trace!("data not found for key={}", key);
            Ok(None)
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        debug!("removing data by key={}", key);

        if self.inner.del(key.as_bytes())?.is_some() {
            trace!("removed success: key={}", key);
            Ok(())
        } else {
            trace!("removing non-exist data: key={}", key);
            Err(Error::KeyNotFound(key))
        }
    }
}
