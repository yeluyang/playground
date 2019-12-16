#![deny(missing_docs)]

//! kvs

use std::collections::HashMap;

/// KvStore
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// create an empty object of KvStore and return it
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }
    /// insert a key-value into KvStore
    ///
    /// # Arguments
    /// - key: String type
    /// - value: String type
    ///
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// # fn main() {
    /// let key = String::from("key");
    /// let value = String::from("value");
    /// let mut kvs = KvStore::new();
    /// kvs.set(key.clone(), value.clone());
    /// assert_eq!(kvs.get(key).unwrap(), value);
    /// # }
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    /// get the value of key
    ///
    /// # Arguments
    /// - key: String type
    ///
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// # fn main() {
    /// let key = String::from("key");
    /// let value = String::from("value");
    ///
    /// let mut kvs = KvStore::new();
    /// assert_eq!(kvs.get(key.to_owned()), None);
    ///
    /// kvs.set(key.clone(), value.clone());
    /// assert_eq!(kvs.get(key).unwrap(), value);
    /// # }
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        match self.data.get(&key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }
    /// remove a key-value from object of KvStore
    ///
    /// # Arguments
    /// - key: String type
    ///
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// # fn main() {
    /// let key = String::from("key");
    /// let mut kvs = KvStore::new();
    /// kvs.remove(key);
    /// # }
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
