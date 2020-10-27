mod error;
mod seg_file;
mod segment;

#[cfg(test)]
mod tests;

extern crate byteorder;
use byteorder::LittleEndian;
type Endian = LittleEndian;

#[macro_use]
extern crate log;

pub use error::{Error, Result};
pub use seg_file::BytesIO;
