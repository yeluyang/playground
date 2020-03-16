use crate::errors::Result;

mod sled_db;
pub use sled_db::SledKvsEngine;

mod kvs;
pub use self::kvs::KvStore;

pub trait KvsEngine: Clone + Send + 'static {
    fn set(&self, key: String, value: String) -> Result<()>;
    fn get(&self, key: String) -> Result<Option<String>>;
    fn remove(&self, key: String) -> Result<()>;
}
