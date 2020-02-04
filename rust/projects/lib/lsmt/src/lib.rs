use std::{collections::VecDeque, fs, ops::RangeInclusive, path::Path};

mod error;
pub use error::{Error, Result};

mod lsf;
pub use lsf::{LogEntryPointer, Record};
use lsf::{LogFileHeader, LogStructuredFile};

#[cfg(test)]
mod tests;

pub struct Config {
    pub lsmt_dir: String,
    pub file_size: usize,
    pub merge_threshold: usize,
}

pub struct LogStructuredMergeTree {
    cfg: Config,

    fds: VecDeque<LogStructuredFile>,

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
            if path.ends_with(".wal") {
                fds.push(LogStructuredFile::open(path)?);
            }
        }
        if fds.is_empty() {
            fds.push(LogStructuredFile::create(
                &cfg.lsmt_dir,
                LogFileHeader::new(RangeInclusive::new(0, 0), false),
            )?)
        }
        fds.sort();
        for fd in &mut fds {
            fd.compact()?;
        }
        let fds = VecDeque::from(fds);
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
        self.fds
            .back_mut()
            .expect("no *.wal files in directory")
            .append(r)
    }

    fn merge() {
        unimplemented!()
    }
}
