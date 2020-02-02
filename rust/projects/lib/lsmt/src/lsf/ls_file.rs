use std::{
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

    pub fn create<P: AsRef<Path>>(path: P, header: LogFileHeader) -> Result<LogStructuredFile> {
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

    fn write_end(&mut self, l: LogEntry) -> Result<()> {
        self.fd.append(l.to_bytes().as_slice())?;
        Ok(())
    }

    pub fn pop<T: Record>(&mut self) -> Result<Option<T>> {
        while let Some(l) = self.read_next()? {
            match l {
                LogEntry::Data(data) => return Ok(Some(T::from(data.data))),
                _ => continue, // skip log file header and log index
            }
        }
        Ok(None)
    }

    pub fn pop_pointer<T: Record>(&mut self) -> Result<Option<LogEntryPointer>> {
        Ok(self.pop::<T>()?.map(|r| LogEntryPointer {
            file_id: *self.header.ids.start(),
            entry_key: r.get_entry_key(),
        }))
    }

    pub fn append<T: Record>(&mut self, r: &T) -> Result<()> {
        self.write_end(LogEntry::from(r))
    }

    fn read_by_seek<T: Record>(&mut self, n: usize) -> Result<Option<T>> {
        self.fd.seek_header(n + 1)?; // +1 to skip log file header
        self.pop()
    }

    pub fn read_by_pointer<T: Record>(&mut self, p: &LogEntryPointer) -> Result<Option<T>> {
        self.fd.seek_header(1)?;
        while let Some(r) = self.pop::<T>()? {
            if r.get_entry_key() == p.entry_key {
                return Ok(Some(r));
            }
        }
        Ok(None)
    }
}
