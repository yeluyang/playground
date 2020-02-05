use std::{fs, ops::RangeInclusive, path::Path};

mod error;
pub use error::{Error, Result};

mod lsf;
pub use lsf::{LogEntryPointer, Record};
use lsf::{LogFileHeader, LogStructuredFile};

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Config {
    pub lsmt_dir: String,
    pub file_size: usize,
    pub merge_threshold: usize,
}

pub struct LogStructuredMergeTree {
    cfg: Config,

    fds: Vec<LogStructuredFile>,

    fd_cursor: usize,
}

impl LogStructuredMergeTree {
    pub fn open(cfg: Config) -> Result<Self> {
        let lsmt_dir = Path::new(&cfg.lsmt_dir);
        if !lsmt_dir.exists() {
            fs::create_dir_all(lsmt_dir)?;
        }
        let mut fds = vec![];
        for entry in fs::read_dir(lsmt_dir)? {
            let path = entry?.path();
            if let Some(ext) = path.extension() {
                if ext == "wal" {
                    fds.push(LogStructuredFile::open(path)?);
                }
            }
        }
        if fds.is_empty() {
            fds.push(LogStructuredFile::create(
                &cfg.lsmt_dir,
                LogFileHeader::new(RangeInclusive::new(0, 0), false),
            )?)
        }
        fds.sort();
        // TODO
        // for fd in &mut fds {
        //     fd.compact()?;
        // }
        Ok(LogStructuredMergeTree {
            cfg,
            fds,
            fd_cursor: 0,
        })
    }

    pub fn pop(&mut self) -> Result<Option<LogEntryPointer>> {
        loop {
            if let Some(p) = self.fds[self.fd_cursor].pop_pointer()? {
                return Ok(Some(p));
            } else if self.fd_cursor == self.fds.len() - 1 {
                return Ok(None);
            } else {
                self.fd_cursor += 1;
            }
        }
    }

    pub fn append<T: Record>(&mut self, r: &T) -> Result<LogEntryPointer> {
        if self.fds.last().unwrap().entry_count >= 2 * (self.cfg.file_size + 1) {
            let next_id = *self.fds.last().unwrap().header.ids.end() + 1;
            self.fds.push(LogStructuredFile::create(
                &self.cfg.lsmt_dir,
                LogFileHeader::new(RangeInclusive::new(next_id, next_id), false),
            )?)
        }
        self.fds.last_mut().unwrap().append(r)
    }

    pub fn read_by_pointer<T: Record>(&mut self, p: &LogEntryPointer) -> Result<Option<T>> {
        for fd in &mut self.fds {
            if fd.header.ids.contains(&p.file_id) {
                return fd.read_by_pointer(p);
            }
        }
        Ok(None)
    }

    fn merge() {
        unimplemented!()
    }
}
