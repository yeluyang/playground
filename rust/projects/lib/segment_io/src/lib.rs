use std::{
    fs::{File, OpenOptions},
    io::{self, Cursor, Read, Seek, SeekFrom, Write},
    mem,
    path::Path,
};

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[cfg(test)]
mod tests;

type Endian = LittleEndian;

const HEADER_SIZE: usize = mem::size_of::<Header>();
#[derive(Default, Debug, PartialEq)]
struct Header {
    length: u128,
    seq_id: u128,
    total: u128,
}

impl Header {
    pub fn new(data_len: usize) -> Self {
        Header {
            length: data_len as u128,
            seq_id: 0,
            total: 0,
        }
    }

    pub fn from(bs: &[u8]) -> io::Result<Self> {
        assert_eq!(bs.len(), HEADER_SIZE);

        let mut h = Header::default();

        let mut rdr = Cursor::new(Vec::from(bs));

        h.length = rdr.read_u128::<Endian>()?;
        h.seq_id = rdr.read_u128::<Endian>()?;
        h.total = rdr.read_u128::<Endian>()?;

        Ok(h)
    }

    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut wtr = Vec::new();

        wtr.write_u128::<Endian>(self.length)?;
        wtr.write_u128::<Endian>(self.seq_id)?;
        wtr.write_u128::<Endian>(self.total)?;

        if wtr.len() != HEADER_SIZE {
            panic!(
                "length of Header's bytes={}, expected={}",
                wtr.len(),
                HEADER_SIZE
            );
        }

        Ok(wtr)
    }
}

pub struct SegmentFile {
    inner: File,
    // TODO add array cache pair: header to pos
}

impl SegmentFile {
    pub fn new(mut fd: File) -> io::Result<SegmentFile> {
        fd.seek(SeekFrom::Start(0))?;
        Ok(SegmentFile { inner: fd })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<SegmentFile> {
        SegmentFile::new(
            OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(path)?,
        )
    }

    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<SegmentFile> {
        SegmentFile::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)?,
        )
    }

    fn read_header(&mut self) -> io::Result<Option<Header>> {
        let h_buf: &mut [u8] = &mut [0u8; HEADER_SIZE];
        let len = self.inner.read(h_buf)?;
        if len == 0 {
            return Ok(None);
        } else if len != HEADER_SIZE {
            panic!(
                "length of bytes from file mismatch header size: {} vs {}",
                len, HEADER_SIZE
            );
        }
        let h = Header::from(h_buf)?;

        Ok(Some(h))
    }

    pub fn pop(&mut self) -> io::Result<Option<Vec<u8>>> {
        if let Some(h) = self.read_header()? {
            let mut data = vec![0u8; h.length as usize];
            self.inner.read_exact(data.as_mut_slice())?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn append(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.seek(SeekFrom::End(0))?;
        let h = Header::new(buf.len());
        self.inner.write_all(h.to_bytes()?.as_slice())?;
        Ok(self.inner.write(buf)?)
    }

    pub fn seek_header(&mut self, n: usize) -> io::Result<u64> {
        self.inner.seek(SeekFrom::Start(0))?;

        if n <= 1 {
            return Ok(0u64);
        }

        let mut i = 1usize;
        let mut len = 0u64;
        while let Some(h) = self.read_header()? {
            len = self.inner.seek(SeekFrom::Current(h.length as i64))?;
            if i == n - 1 {
                break;
            }
            i += 1;
        }
        Ok(len)
    }
}

impl Read for SegmentFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(data) = self.pop()? {
            let len = data.len();
            if buf.len() < len as usize {
                panic!(
                    "length of buffer={}, at least greater than {}",
                    buf.len(),
                    len
                );
            }
            buf[..len].clone_from_slice(&data[..len]);
            Ok(len)
        } else {
            Ok(0)
        }
    }
}

impl Write for SegmentFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.append(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl Seek for SegmentFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let p = match pos {
            SeekFrom::Start(p) => p as usize,
            _ => unimplemented!(),
        };
        self.seek_header(p)
    }
}
