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

    pub fn from(bs: &[u8]) -> Self {
        assert_eq!(bs.len(), HEADER_SIZE);

        let mut h = Header::default();

        let mut rdr = Cursor::new(Vec::from(bs));

        h.length = rdr.read_u128::<Endian>().unwrap();
        h.seq_id = rdr.read_u128::<Endian>().unwrap();
        h.total = rdr.read_u128::<Endian>().unwrap();

        h
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut wtr = Vec::new();

        wtr.write_u128::<Endian>(self.length).unwrap();
        wtr.write_u128::<Endian>(self.seq_id).unwrap();
        wtr.write_u128::<Endian>(self.total).unwrap();

        if wtr.len() != HEADER_SIZE {
            panic!(
                "length of Header's bytes={}, expected={}",
                wtr.len(),
                HEADER_SIZE
            );
        }

        wtr
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
}

impl Read for SegmentFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let h_buf: &mut [u8] = &mut [0u8; HEADER_SIZE];
        self.buff.read_exact(h_buf)?;
        let h = Header::from(h_buf);

        if buf.len() < h.length as usize {
            panic!("length of buffer={}, expected={}", buf.len(), h.length);
        }

        let mut data = vec![0u8; h.length as usize];
        let len = self.buff.read(data.as_mut_slice())?;
        buf[..len].clone_from_slice(&data[..len]);
        Ok(len)
    }
}

impl Write for SegmentFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let h = Header::new(buf.len());
        self.buff.write_all(h.to_bytes().as_slice())?;
        let len = self.buff.write(buf)?;
        Ok(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.buff.flush()
    }
}
