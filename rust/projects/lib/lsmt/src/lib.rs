use std::{fs, ops::RangeInclusive, path::Path};

mod error;
pub use error::{Error, Result};

mod lsf;
use lsf::{LogEntry, LogEntryKey, LogFileHeader, LogStructuredFile};
pub use lsf::{LogEntryPointer, Record};

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Config {
    pub lsmt_dir: String,
    pub file_size: usize,
    pub compact_enable: bool,
    pub merge_threshold: Option<usize>,
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
        if cfg.compact_enable {
            for fd in fds.split_last_mut().unwrap().1 {
                fd.compact()?;
            }
        }
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
            if self.cfg.compact_enable {
                self.fds.last_mut().unwrap().compact()?;
            }

            let next_id = *self.fds.last().unwrap().header.ids.end() + 1;
            self.fds.push(LogStructuredFile::create(
                &self.cfg.lsmt_dir,
                LogFileHeader::new(RangeInclusive::new(next_id, next_id), false),
            )?);
        }
        if let Some(threshold) = self.cfg.merge_threshold {
        if self.fds.len() - 1 > self.cfg.merge_threshold {
                self.merge()?;
            }
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

    fn merge(&mut self) -> Result<()> {
        self.fds.reverse();
        let mut old_fds = self.fds.split_off(1);
        // create new file with id [id .. id]
        let mut fd = LogStructuredFile::create(
            &self.cfg.lsmt_dir,
            LogFileHeader::new(
                RangeInclusive::new(
                    *old_fds.last().unwrap().header.ids.start(),
                    *old_fds.first().unwrap().header.ids.end(),
                ),
                false,
            ),
        )?;
        // append entry into new file from other compacted files
        while let Some(mut old) = old_fds.pop() {
            //for old in old_fds.iter_mut() {
            let mut entrys: Vec<(usize, LogEntryKey, LogEntry)> = Vec::new();
            for (key, count) in &old.index.clone() {
                let l = old
                    .read_entry_by_pointer(&LogEntryPointer::new(
                        *old.header.ids.start(),
                        key.clone(),
                    ))?
                    .unwrap();
                entrys.push((*count, key.clone(), l));
            }
            entrys.sort_by(|l, r| l.0.cmp(&r.0));
            for e in entrys {
                fd.write_entry_data(e.1, e.2)?;
            }
            // remove older fd with its local file
            fs::remove_file(old.path())?;
        }
        // compact new file
        fd.compact()?;
        // push new file into self.fds front
        self.fds.insert(0, fd);
        Ok(())
    }
}
