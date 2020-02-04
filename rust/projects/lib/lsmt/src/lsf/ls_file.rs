use std::{
    cmp::Ordering,
    fs::{File, OpenOptions},
    path::Path,
};

extern crate segment_io;
use segment_io::SegmentFile;

use crate::error::{self, Error, Result};

use super::entry::{LogEntry, LogEntryPointer, LogFileHeader, Record};

pub struct LogStructuredFile {
    header: LogFileHeader,
    fd: SegmentFile,
}

impl LogStructuredFile {
    pub fn new(fd: File, file_header: LogFileHeader) -> Result<LogStructuredFile> {
        Ok(LogStructuredFile {
            header: file_header,
            fd: SegmentFile::new(fd)?,
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<LogStructuredFile> {
        let mut ls_fd = LogStructuredFile::new(
            OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(&path)?,
            LogFileHeader::default(),
        )?;

        if let Some(l) = ls_fd.read_next()? {
            match l {
                LogEntry::FileHeader(h) => {
                    ls_fd.header = h;
                    Ok(ls_fd)
                }
                _ => Err(Error::HeaderMissing(error::get_path_string(path)?)),
            }
        } else {
            Err(Error::EmptyFile(error::get_path_string(path)?))
        }
    }

    pub fn create<P: AsRef<Path>>(dir: P, header: LogFileHeader) -> Result<LogStructuredFile> {
        let path = dir.as_ref().join(format!("{}.wal", header.ids.end()));
        let mut ls_fd = LogStructuredFile::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)?,
            header,
        )?;
        ls_fd.write_end(LogEntry::FileHeader(ls_fd.header.clone()))?;
        Ok(ls_fd)
    }

    fn read_next(&mut self) -> Result<Option<LogEntry>> {
        Ok(self.fd.pop()?.map(LogEntry::from))
    }

    fn read_next_data(&mut self) -> Result<Option<LogEntry>> {
        while let Some(l) = self.read_next()? {
            match l {
                LogEntry::Data(_) => return Ok(Some(l)),
                _ => continue, // skip log file header and log index
            }
        }
        Ok(None)
    }

    fn write_end(&mut self, l: LogEntry) -> Result<()> {
        self.fd.append(l.to_bytes().as_slice())?;
        Ok(())
    }

    pub fn pop<T: Record>(&mut self) -> Result<Option<T>> {
        if let Some(l) = self.read_next_data()? {
            match l {
                LogEntry::Data(data) => Ok(Some(T::from(data.data))),
                _ => unreachable!(),
            }
        } else {
            Ok(None)
        }
    }

    pub fn pop_pointer(&mut self) -> Result<Option<LogEntryPointer>> {
        if let Some(l) = self.read_next_data()? {
            match l {
                LogEntry::Data(data) => Ok(Some(LogEntryPointer::new(
                    *self.header.ids.start(),
                    data.key,
                ))),
                _ => unreachable!(),
            }
        } else {
            Ok(None)
        }
    }

    pub fn append<T: Record>(&mut self, r: &T) -> Result<LogEntryPointer> {
        self.write_end(LogEntry::from(r))?;
        Ok(LogEntryPointer::new(
            *self.header.ids.start(),
            r.get_entry_key(),
        ))
    }

    fn read_by_seek<T: Record>(&mut self, n: usize) -> Result<Option<T>> {
        self.fd.seek_segment(n + 1)?; // +1 to skip log file header
        self.pop()
    }

    pub fn read_by_pointer<T: Record>(&mut self, p: &LogEntryPointer) -> Result<Option<T>> {
        self.fd.seek_segment(1)?;
        while let Some(r) = self.pop::<T>()? {
            if r.get_entry_key() == p.entry_key {
                return Ok(Some(r));
            }
        }
        Ok(None)
    }

    pub fn compact(&mut self) -> Result<()> {
        unimplemented!()
    }
}

impl Eq for LogStructuredFile {}

impl PartialEq for LogStructuredFile {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
    }
}

impl PartialOrd for LogStructuredFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogStructuredFile {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.header.ids.end() < other.header.ids.end() {
            Ordering::Less
        } else if self.eq(other) {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
