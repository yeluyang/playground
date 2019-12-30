#![deny(missing_docs)]

//! kvs

use std::{collections::HashMap, path::Path};

mod errors;
pub use errors::{Error, Result};

/// KvStore
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// create an empty object of KvStore and return it
    pub fn new() -> KvStore {
        Default::default()
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
        self.data.insert(key, value);
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
        self.data
            .remove(&key)
            .ok_or(Error::KeyNotFound(key))
            .map(|_| ())
    }
}
