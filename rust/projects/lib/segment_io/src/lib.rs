mod seg_file;
mod segment;

#[cfg(test)]
mod tests;

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

type Endian = LittleEndian;

#[macro_use]
extern crate log;

pub use seg_file::SegmentFile;
