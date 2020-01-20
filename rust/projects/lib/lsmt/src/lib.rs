mod error;

mod record;
pub use record::Record;

#[cfg(test)]
mod tests;

use std::{
    fs::{File, OpenOptions},
    path::Path,
};

pub use error::{Error, Result};

extern crate segment_io;
use segment_io::SegmentFile;

pub struct LogStructuredMergeTree {
    fd: SegmentFile,
}

impl LogStructuredMergeTree {
    pub fn new(fd: File) -> Result<LogStructuredMergeTree> {
        Ok(LogStructuredMergeTree {
            fd: SegmentFile::new(fd)?,
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<LogStructuredMergeTree> {
        LogStructuredMergeTree::new(
            OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(path)?,
        )
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<LogStructuredMergeTree> {
        LogStructuredMergeTree::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)?,
        )
    }

    pub fn read_next<T: From<Vec<u8>>>(&mut self) -> Result<Option<T>> {
        Ok(self.fd.pop()?.map(T::from))
    }

    pub fn append<T: Record>(&mut self, r: &T) -> Result<()> {
        self.fd.append(r.to_bytes().as_slice())?;
        Ok(())
    }
}
