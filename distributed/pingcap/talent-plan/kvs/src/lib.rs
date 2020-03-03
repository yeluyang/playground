// #![deny(missing_docs)]

//! kvs

#[macro_use]
extern crate log;

mod errors;
pub use errors::{Error, Result};

mod command;

pub mod engines;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};

pub mod network;
