use std::path::Path;

extern crate sled;
use sled::Db;

use crate::errors::Result;

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

        if let Some(val) = self.inner.get(key)? {
            Ok(Some(String::from_utf8(val.to_vec())?))
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        debug!("removing data by key={}", key);

        self.inner.del(key)?;
        Ok(())
    }
}
