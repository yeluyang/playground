use std::{
    cmp::Ordering,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

extern crate segment_io;
use segment_io::SegmentFile;

use crate::error::{self, Error, Result};

use super::entry::{LogEntry, LogEntryIndex, LogEntryPointer, LogFileHeader, Record};

pub struct LogStructuredFile {
    path: String,
    pub(crate) header: LogFileHeader,
    pub(crate) index: LogEntryIndex,
    pub(crate) entry_count: usize,
    fd: SegmentFile,
}

impl LogStructuredFile {
    pub fn new<P: AsRef<Path>>(
        path: P,
        fd: SegmentFile,
        file_header: LogFileHeader,
    ) -> Result<LogStructuredFile> {
        Ok(LogStructuredFile {
            path: error::path_to_string(path)?,
            header: file_header,
            index: LogEntryIndex::default(),
            entry_count: 0,
            fd,
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<LogStructuredFile> {
        let mut ls_fd =
            LogStructuredFile::new(&path, SegmentFile::open(&path)?, LogFileHeader::default())?;

        if let Some(l) = ls_fd.read_next()? {
            match l {
                LogEntry::FileHeader(h) => {
                    ls_fd.header = h;
                }
                _ => return Err(Error::HeaderMissing(error::path_to_string(path)?)),
            }
        } else {
            return Err(Error::EmptyFile(error::path_to_string(path)?));
        };

        ls_fd.fd.seek(SeekFrom::End(0))?;
        match ls_fd.read_next()?.unwrap() {
            LogEntry::Index(entry_count, index) => {
                ls_fd.index = index;
                ls_fd.entry_count = entry_count;
            }
            _ => return Err(Error::IncompleteWrite(error::path_to_string(path)?)),
        };

        ls_fd.fd.seek(SeekFrom::Start(0))?;
        Ok(ls_fd)
    }

    pub fn create<P: AsRef<Path>>(dir: P, header: LogFileHeader) -> Result<LogStructuredFile> {
        let path = dir.as_ref().join(format!("{}.wal", header.ids.end()));
        let mut ls_fd = LogStructuredFile::new(&path, SegmentFile::create(&path)?, header)?;
        ls_fd.write_end(LogEntry::FileHeader(ls_fd.header.clone()))?;
        ls_fd.write_end(LogEntry::Index(ls_fd.entry_count + 1, ls_fd.index.clone()))?;
        Ok(ls_fd)
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    fn read_next(&mut self) -> Result<Option<LogEntry>> {
        Ok(self.fd.pop()?.map(LogEntry::from))
    }

    fn read_next_record(&mut self) -> Result<Option<LogEntry>> {
        while let Some(l) = self.read_next()? {
            match l {
                LogEntry::Data(_) => return Ok(Some(l)),
                _ => continue, // skip log file header and log index
            }
        }
        Ok(None)
    }

    pub fn pop<T: Record>(&mut self) -> Result<Option<T>> {
        if let Some(l) = self.read_next_record()? {
            match l {
                LogEntry::Data(data) => Ok(Some(T::from(data.data))),
                _ => unreachable!(),
            }
        } else {
            Ok(None)
        }
    }

    pub fn pop_pointer(&mut self) -> Result<Option<LogEntryPointer>> {
        if let Some(l) = self.read_next_record()? {
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

    fn write_end(&mut self, l: LogEntry) -> Result<()> {
        self.fd.append(l.to_bytes().as_slice())?;
        self.fd.flush()?;
        self.entry_count += 1;
        Ok(())
    }

    pub fn append<T: Record>(&mut self, r: &T) -> Result<LogEntryPointer> {
        self.write_end(LogEntry::from(r))?;
        self.index.insert(r.key(), self.entry_count - 1);
        self.write_end(LogEntry::Index(self.entry_count + 1, self.index.clone()))?;
        Ok(LogEntryPointer::new(*self.header.ids.start(), r.key()))
    }

    pub fn read_by_pointer<T: Record>(&mut self, p: &LogEntryPointer) -> Result<Option<T>> {
        let header_count = self.index[&p.key];
        self.fd.seek_segment(header_count)?;
        self.pop()
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
