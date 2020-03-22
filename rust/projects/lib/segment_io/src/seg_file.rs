use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write},
    mem,
    ops::Range,
    path::Path,
};

extern crate byteorder;
use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{
    error::{Error, Result},
    segment::{self, Segment},
    Endian,
};

#[derive(Debug, Clone, Default, PartialEq)]
struct FileHeader {
    header_bytes: u128,
    payload_bytes: u128,
}
const FILE_HEADER_SIZE: usize = mem::size_of::<FileHeader>();

impl FileHeader {
    fn new(payload_bytes: u128) -> Self {
        Self {
            header_bytes: segment::SEGMENT_HEADER_SIZE as u128,
            payload_bytes,
        }
    }
    fn from(bs: &[u8]) -> io::Result<Self> {
        assert_eq!(bs.len(), FILE_HEADER_SIZE);

        let mut rdr = Cursor::new(Vec::from(bs));
        let header_bytes = rdr.read_u128::<Endian>()?;
        let payload_bytes = rdr.read_u128::<Endian>()?;

        Ok(Self {
            header_bytes,
            payload_bytes,
        })
    }

    fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut wtr = Vec::new();
        wtr.write_u128::<Endian>(self.header_bytes)?;
        wtr.write_u128::<Endian>(self.payload_bytes)?;

        if wtr.len() != FILE_HEADER_SIZE {
            panic!(
                "length of Version's bytes={}, expected={}",
                wtr.len(),
                FILE_HEADER_SIZE,
            );
        }

        Ok(wtr)
    }
}

#[derive(Debug)]
pub struct SegmentFile {
    header: FileHeader,
    reader: BufReader<File>,
    writer: BufWriter<File>,

    segment_bytes: usize,
    segment_seq: usize,
}

impl SegmentFile {
    pub fn create<P: AsRef<Path>>(path: P, payload_bytes: u128) -> Result<Self> {
        debug!("create segment file: {:?}", path.as_ref());

        if path.as_ref().exists() {
            // TODO: add error
            panic!("already exists: {:?}", path.as_ref());
        }

        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.as_ref())?;
        let reader = OpenOptions::new().read(true).open(path)?;

        let mut seg = Self {
            segment_seq: 0,

            header: FileHeader::new(payload_bytes),
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            segment_bytes: payload_bytes as usize + segment::SEGMENT_HEADER_SIZE,
        };
        trace!("header of new segment file, header={:?}", seg.header);

        seg.writer.seek(SeekFrom::Start(0))?;
        seg.writer.write_all(seg.header.to_bytes()?.as_slice())?;
        seg.reader.seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;

        trace!("inited: SegmentFile={:?}", seg);
        Ok(seg)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        debug!("open segment file: {:?}", path.as_ref());

        let writer = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.as_ref())?;
        let mut reader = OpenOptions::new().read(true).open(path)?;

        let mut buf: [u8; FILE_HEADER_SIZE] = [0u8; FILE_HEADER_SIZE];
        let bytes = reader.read(&mut buf)?;
        if bytes == 0 {
            // TODO add error
            panic!("missing header of segment file");
        };

        let header = FileHeader::from(&buf)?;
        trace!("file-header={:?}", header);
        let segment_bytes = (header.header_bytes + header.payload_bytes) as usize;

        let mut seg = Self {
            segment_seq: 0,

            header,
            segment_bytes,
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        };

        seg.reader.seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;
        seg.writer.seek(SeekFrom::End(0))?;

        let segments_bytes = seg.writer.seek(SeekFrom::Current(0))? as usize - FILE_HEADER_SIZE;
        trace!("bytes of segments of file: bytes={}", segments_bytes);
        if segments_bytes == 0 {
            seg.segment_seq = 0;
        } else {
            assert_eq!(segments_bytes % seg.segment_bytes, 0);
            seg.segment_seq = segments_bytes / seg.segment_bytes;
        }

        trace!("inited: SegmentFile={:?}", seg);
        Ok(seg)
    }

    fn next_segment(&mut self) -> Result<Option<Segment>> {
        debug!("reading next segment");
        let mut buf: Vec<u8> = vec![0u8; self.segment_bytes];
        self.reader.read_exact(buf.as_mut_slice())?;

        if buf.is_empty() {
            trace!("read EOF");
            Ok(None)
        } else if buf.len() != self.segment_bytes {
            panic!(
                "length of bytes from file mismatch header size: {} vs {}",
                buf.len(),
                self.segment_bytes
            );
        } else {
            let segment = Segment::from(buf.as_slice())?;
            trace!("read next segment success: segment={:?}", segment);
            Ok(Some(segment))
        }
    }

    // TODO add `next_n_segments(n)->Result<Option<Vec<Segment>>>`

    pub fn seek_segment(&mut self, n: usize) -> Result<Option<u64>> {
        debug!("seeking segment: segment_seq={}", n);

        let bytes = FILE_HEADER_SIZE + self.segment_bytes * n;
        trace!("seek bytes from start: bytes={}", bytes);
        if self.reader.seek(SeekFrom::Start(bytes as u64))? as usize == bytes {
            Ok(Some(bytes as u64))
        } else {
            error!("encounter EOF when seeking");
            Ok(None)
        }
    }

    pub fn pop(&mut self) -> Result<Option<Vec<u8>>> {
        debug!("reading full content");

        let mut bytes: Vec<u8> = Vec::new();
        if let Some(seg) = self.next_segment()? {
            if seg.is_first() {
                if seg.is_last() {
                    trace!(
                        "read all content success, first is all: content={:?}",
                        seg.payload()
                    );
                    return Ok(Some(seg.payload()));
                } else {
                    bytes.extend(seg.payload());
                }
            } else {
                // TODO return error
                unimplemented!();
            }
        } else {
            return Ok(None);
        };
        // TODO use `next_n_segments`
        while let Some(seg) = self.next_segment()? {
            let is_last = seg.is_last();
            bytes.extend(seg.payload());
            if is_last {
                trace!("read all content success: content={:?}", bytes);
                return Ok(Some(bytes));
            }
        }
        panic!("incomplete write")
    }

    pub fn append(&mut self, buf: &[u8]) -> Result<Range<usize>> {
        debug!(
            "writting data into segment_file: data={{len={}, val={:?}}}",
            buf.len(),
            buf
        );
        let segs = segment::create(buf.to_owned(), self.header.payload_bytes as usize);
        trace!(
            "create a segments with data: segments={{len={}, val={:?}}}",
            segs.len(),
            segs
        );
        let segment_seq = self.segment_seq;
        for seg in segs {
            self.writer.write_all(seg.to_bytes()?.as_slice())?;
            self.segment_seq += 1;
        }

        trace!(
            "write all success: segment_file.segment_seq={{before={}, now={}}}",
            segment_seq,
            self.segment_seq
        );
        Ok(Range {
            start: segment_seq,
            end: self.segment_seq,
        })
    }

    // TODO add `replace`
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use super::*;

    extern crate serde;
    use serde::{Deserialize, Serialize};

    extern crate serde_json;

    extern crate env_logger;
    use env_logger::{Builder, Env};

    static INIT: std::sync::Once = std::sync::Once::new();
    fn init() {
        INIT.call_once(|| {
            Builder::from_env(Env::default().default_filter_or("trace"))
                .is_test(true)
                .init();
        })
    }

    #[test]
    fn test_file_header() {
        init();
        struct Case {
            header: FileHeader,
        }
        let cases = [Case {
            header: FileHeader::new(128),
        }];

        for c in cases.iter() {
            let bytes = c.header.to_bytes().unwrap();
            assert_eq!(bytes.len(), FILE_HEADER_SIZE);
            let header = FileHeader::from(bytes.as_slice()).unwrap();
            assert_eq!(header, c.header);
            assert_eq!(header.to_bytes().unwrap(), bytes);
        }
    }

    #[test]
    fn test_segment_file() {
        init();
        const TEST_PAYLOAD_BYTES: usize = 64;

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Case {
            id: usize,
            data: Vec<usize>,
        }
        let cases = [
            Case {
                id: 0,
                data: vec![0; TEST_PAYLOAD_BYTES / 4],
            },
            Case {
                id: 1,
                data: vec![1; TEST_PAYLOAD_BYTES / 2],
            },
            Case {
                id: 2,
                data: vec![2; TEST_PAYLOAD_BYTES],
            },
            Case {
                id: 3,
                data: vec![3; TEST_PAYLOAD_BYTES * 2],
            },
            Case {
                id: 4,
                data: vec![4; TEST_PAYLOAD_BYTES * 4],
            },
        ];
        let mut index: HashMap<usize, Range<usize>> = HashMap::new();

        let tmp_dir = Path::new("tmp");
        if tmp_dir.exists() {
            fs::remove_dir_all(tmp_dir).unwrap();
        }
        fs::create_dir(tmp_dir).unwrap();

        {
            let mut s_file =
                SegmentFile::create("tmp/tmp.segment", TEST_PAYLOAD_BYTES as u128).unwrap();
            assert_eq!(s_file.segment_seq, 0);
            assert_eq!(
                s_file.segment_bytes,
                segment::SEGMENT_HEADER_SIZE + TEST_PAYLOAD_BYTES
            );
            assert_eq!(
                s_file.header.header_bytes as usize,
                segment::SEGMENT_HEADER_SIZE
            );
            assert_eq!(s_file.header.payload_bytes as usize, TEST_PAYLOAD_BYTES);

            for (i, case) in cases.iter().enumerate() {
                let bytes = serde_json::to_vec(case).unwrap();
                let seq_rng = s_file.append(bytes.as_slice()).unwrap();
                index.insert(i, seq_rng);
            }
            assert_eq!(index.len(), cases.len());
        }

        let mut s_file = SegmentFile::open("tmp/tmp.segment").unwrap();
        assert_eq!(s_file.segment_seq, index[&(cases.len() - 1)].end);
        assert_eq!(
            s_file.segment_bytes,
            segment::SEGMENT_HEADER_SIZE + TEST_PAYLOAD_BYTES
        );
        assert_eq!(
            s_file.header.header_bytes as usize,
            segment::SEGMENT_HEADER_SIZE
        );
        assert_eq!(s_file.header.payload_bytes as usize, TEST_PAYLOAD_BYTES);

        for case in &cases {
            let bytes_json = serde_json::to_vec(case).unwrap();
            let bytes_seg = s_file.pop().unwrap().unwrap();
            assert_eq!(bytes_seg, bytes_json);

            let c: Case = serde_json::from_slice(bytes_seg.as_slice()).unwrap();
            assert_eq!(&c, case);
        }

        for (i, case) in cases.iter().enumerate() {
            let seek_bytes = s_file.seek_segment(index[&i].start).unwrap().unwrap();
            assert_eq!(
                seek_bytes,
                (FILE_HEADER_SIZE + s_file.segment_bytes * index[&i].start) as u64
            );

            let js_bytes = serde_json::to_vec(case).unwrap();
            let seg_bytes = s_file.pop().unwrap().unwrap();
            assert_eq!(seg_bytes, js_bytes);

            let c: Case = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
            assert_eq!(&c, case);
        }

        for (i, case) in cases.iter().rev().enumerate() {
            let seek_bytes = s_file
                .seek_segment(index[&(cases.len() - i - 1)].start)
                .unwrap()
                .unwrap();
            assert_eq!(
                seek_bytes,
                (FILE_HEADER_SIZE + s_file.segment_bytes * index[&(cases.len() - i - 1)].start)
                    as u64
            );
            let js_bytes = serde_json::to_vec(case).unwrap();
            let seg_bytes = s_file.pop().unwrap().unwrap();
            assert_eq!(seg_bytes, js_bytes);

            let c: Case = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
            assert_eq!(&c, case);
        }
    }
}
