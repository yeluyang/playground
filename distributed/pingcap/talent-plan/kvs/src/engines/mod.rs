use crate::errors::Result;

mod kvs;
pub use kvs::KvStore;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> ;
    fn get(&mut self, key: String) -> Result<Option<String>> ;
    fn remove(&mut self, key: String) -> Result<()>;
}
