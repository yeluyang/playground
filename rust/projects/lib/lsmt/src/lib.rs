mod error;

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
        LogStructuredMergeTree::new(OpenOptions::new().read(true).append(true).open(path)?)
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<LogStructuredMergeTree> {
        LogStructuredMergeTree::new(
            OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(path)?,
        )
    }

    pub fn read_next(&mut self) -> Result<Option<Vec<u8>>> {
        let data = self.fd.pop()?;
        if data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(data))
        }
    }

    pub fn append(&mut self, data: &[u8]) -> Result<()> {
        self.fd.append(data)?;
        Ok(())
    }
}
