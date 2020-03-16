use std::{
    path::Path,
    sync::{Arc, Mutex},
};

extern crate sled;
use sled::Db;

use crate::errors::{Error, Result};

#[derive(Clone)]
pub struct SledKvsEngine {
    inner: Arc<Mutex<Db>>,
}

impl SledKvsEngine {
    /// new
    pub fn new(db: sled::Db) -> Self {
        Self {
            inner: Arc::new(Mutex::new(db)),
        }
    }
    /// open
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<Self> {
        debug!("open sled storage on directory={:?}", dir.as_ref());
        Ok(Self::new(sled::Db::start_default(dir)?))
    }
}

impl super::KvsEngine for SledKvsEngine {
    fn set(&self, key: String, value: String) -> Result<()> {
        debug!("setting data={{key={}, value={}}}", key, value);

        self.inner.lock().unwrap().set(key, value.as_str())?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        debug!("getting data by key={}", key);

        if let Some(val) = self.inner.lock().unwrap().get(&key)? {
            let val = String::from_utf8(val.to_vec())?;
            trace!("found data={{key={}, val={}}}", key, val);
            Ok(Some(val))
        } else {
            trace!("data not found for key={}", key);
            Ok(None)
        }
    }

    fn remove(&self, key: String) -> Result<()> {
        debug!("removing data by key={}", key);

        if self.inner.lock().unwrap().del(key.as_bytes())?.is_some() {
            trace!("removed success: key={}", key);
            Ok(())
        } else {
            trace!("removing non-exist data: key={}", key);
            Err(Error::KeyNotFound(key))
        }
    }
}
