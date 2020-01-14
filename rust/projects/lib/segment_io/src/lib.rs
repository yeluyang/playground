use std::{
    fs::File,
    io::{self, Cursor, Read, Seek, SeekFrom, Write},
    mem,
    path::Path,
};

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

extern crate buff_io;
use buff_io::Buffer;

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
    buff: Buffer<File>,
}

impl SegmentFile {
    pub fn new(fd: File) -> io::Result<SegmentFile> {
        let mut buff = Buffer::new(fd)?;
        buff.seek(SeekFrom::Start(0))?;
        Ok(SegmentFile { buff })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<SegmentFile> {
        SegmentFile::new(File::open(path)?)
    }

    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<SegmentFile> {
        SegmentFile::new(File::create(path)?)
    }

    pub fn pop(&mut self) -> io::Result<Vec<u8>> {
        let h_buf: &mut [u8] = &mut [0u8; HEADER_SIZE];
        self.buff.read_exact(h_buf)?;
        let h = Header::from(h_buf)?;

        let mut data = vec![0u8; h.length as usize];
        self.buff.read_exact(data.as_mut_slice())?;

        Ok(data)
    }

    pub fn append(&mut self, buf: &[u8]) -> io::Result<usize> {
        let h = Header::new(buf.len());
        self.buff.write_all(h.to_bytes()?.as_slice())?;
        let len = self.buff.write(buf)?;
        Ok(len)
    }
}

impl Read for SegmentFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data = self.pop()?;
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
    }
}

impl Write for SegmentFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.append(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.buff.flush()
    }
}
