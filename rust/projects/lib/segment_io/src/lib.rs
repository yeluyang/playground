use std::{
    fs::{File, OpenOptions},
    io::{self, Cursor, Read, Seek, SeekFrom, Write},
    mem,
    ops::Range,
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

#[derive(Debug)]
pub struct SegmentFile {
    inner: File,
    index: Vec<Range<u64>>,
}

impl SegmentFile {
    pub fn new(mut fd: File) -> io::Result<SegmentFile> {
        fd.seek(SeekFrom::Start(0))?;
        Ok(SegmentFile {
            inner: fd,
            index: vec![],
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<SegmentFile> {
        let mut seg = SegmentFile::new(
            OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(path)?,
        )?;
        loop {
            let start = seg.inner.seek(SeekFrom::Current(0))?;
            if let Some(h) = seg.read_header()? {
                let end = seg.inner.seek(SeekFrom::Current(h.length as i64))?;
                seg.index.push(Range { start, end });
            } else {
                break;
            }
        }
        seg.inner.seek(SeekFrom::Start(0))?;
        Ok(seg)
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
            Ok(None)
        } else if len != HEADER_SIZE {
            panic!(
                "length of bytes from file mismatch header size: {} vs {}",
                len, HEADER_SIZE
            );
        } else {
            Ok(Some(Header::from(h_buf)?))
        }
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
        let start = self.inner.seek(SeekFrom::End(0))?;
        let mut end = start;

        let h = Header::new(buf.len());
        end += self.inner.write(h.to_bytes()?.as_slice())? as u64;

        let len = self.inner.write(buf)?;
        end += len as u64;

        self.index.push(Range { start, end });
        Ok(len)
    }

    pub fn seek_segment(&mut self, n: usize) -> io::Result<Option<u64>> {
        if n < self.index.len() {
            self.inner
                .seek(SeekFrom::Start(self.index[n].start))
                .map(Some)
        } else {
            Ok(None)
        }
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
            SeekFrom::End(mut p) => {
                if p < 0 {
                    p *= -1;
                }
                if p as usize >= self.index.len() {
                    return Ok(0);
                }
                self.index.len() - 1 - p as usize
            }
            _ => unimplemented!(),
        };
        Ok(self.seek_segment(p)?.unwrap_or(0))
    }
}
