use std::{
    cmp::Ordering,
    fs,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

extern crate segment_io;
use segment_io::SegmentFile;

use crate::error::{self, Error, Result};

use super::entry::{LogEntry, LogEntryIndex, LogEntryKey, LogEntryPointer, LogFileHeader, Record};

#[derive(Debug)]
pub struct LogStructuredFile {
    path: String,
    pub(crate) header: LogFileHeader,
    pub(crate) index: LogEntryIndex,
    pub(crate) entry_count: usize,
    pub(crate) fd: SegmentFile,
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
        let path = if header.ids.start() == header.ids.end() {
            dir.as_ref().join(format!("{}.wal", header.ids.end()))
        } else {
            dir.as_ref()
                .join(format!("{}-{}.wal", header.ids.start(), header.ids.end()))
        };
        let mut ls_fd = LogStructuredFile::new(&path, SegmentFile::create(&path)?, header)?;
        ls_fd.write_end(LogEntry::FileHeader(ls_fd.header.clone()))?;
        ls_fd.write_end(LogEntry::Index(ls_fd.entry_count, ls_fd.index.clone()))?;
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

    pub(crate) fn write_end(&mut self, l: LogEntry) -> Result<()> {
        self.fd.append(l.to_bytes().as_slice())?;
        self.fd.flush()?;
        self.entry_count += 1;
        Ok(())
    }

    pub(crate) fn write_entry_data(&mut self, key: LogEntryKey, l: LogEntry) -> Result<()> {
        self.index.insert(key, self.entry_count);
        self.write_end(l)?;
        self.write_end(LogEntry::Index(self.entry_count, self.index.clone()))?;
        Ok(())
    }

    pub fn append<T: Record>(&mut self, r: &T) -> Result<LogEntryPointer> {
        self.write_entry_data(r.key(), LogEntry::from(r))?;
        Ok(LogEntryPointer::new(*self.header.ids.start(), r.key()))
    }

    pub(crate) fn read_entry_by_pointer(
        &mut self,
        p: &LogEntryPointer,
    ) -> Result<Option<LogEntry>> {
        let header_count = self.index[&p.key];
        self.fd.seek_segment(header_count)?;
        self.read_next_record()
    }

    pub fn read_by_pointer<T: Record>(&mut self, p: &LogEntryPointer) -> Result<Option<T>> {
        let header_count = self.index[&p.key];
        self.fd.seek_segment(header_count)?;
        self.pop()
    }

    pub fn compact(&mut self) -> Result<()> {
        if self.header.compacted {
            return Ok(());
        }
        let dir = Path::new(&self.path).parent().unwrap();
        let path = if self.header.ids.start() == self.header.ids.end() {
            dir.join(format!("{}.compacted.wal", self.header.ids.end()))
        } else {
            dir.join(format!(
                "{}-{}.compacted.wal",
                self.header.ids.start(),
                self.header.ids.end()
            ))
        };
        let mut compacted = SegmentFile::create(&path)?;

        let mut entry_count = 0usize;

        let mut header = self.header.clone();
        header.compacted = true;
        compacted.append(LogEntry::FileHeader(header.clone()).to_bytes().as_slice())?;
        compacted.flush()?;
        entry_count += 1;

        let mut entrys: Vec<(usize, LogEntryKey, LogEntry)> = Vec::new();
        for (key, header_count) in &self.index {
            self.fd.seek_segment(*header_count)?;
            let l = self.fd.pop()?.map(LogEntry::from).unwrap();
            entrys.push((*header_count, key.clone(), l));
        }
        entrys.sort_by(|l, r| l.0.cmp(&r.0));
        let mut index = LogEntryIndex::new();
        for e in entrys {
            let (_, key, l) = e;
            index.insert(key.clone(), entry_count);
            compacted.append(l.to_bytes().as_slice())?;
            compacted.flush()?;
            entry_count += 1;
        }

        compacted.append(
            LogEntry::Index(entry_count, index.clone())
                .to_bytes()
                .as_slice(),
        )?;
        compacted.flush()?;
        entry_count += 1;

        self.fd = compacted;
        fs::remove_file(&self.path)?;
        self.path = error::path_to_string(path)?;

        self.header = header;
        self.index = index;
        self.entry_count = entry_count;
        Ok(())
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
        if self.header.ids.end() < other.header.ids.start() {
            Ordering::Less
        } else if self.eq(other) {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}