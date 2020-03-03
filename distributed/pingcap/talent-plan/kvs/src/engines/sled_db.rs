use std::path::Path;

extern crate sled;
use sled::Db;

use crate::errors::Result;

pub struct SledDB {
    inner: Db,
}

impl SledDB {
    /// open an object of KvStore on an exist log file and return it
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<SledDB> {
        debug!("open sled storage on directory={:?}", dir.as_ref());
        Ok(SledDB {
            inner: sled::Db::start_default(dir)?,
        })
    }
}

impl super::KvsEngine for SledDB {
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
