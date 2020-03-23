use std::{
    convert::{TryFrom, TryInto},
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write},
    mem,
    ops::Range,
    path::Path,
    result,
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

    fn to_vec_u8(&self) -> Result<Vec<u8>> {
        let mut wtr = Vec::new();
        wtr.write_u128::<Endian>(self.header_bytes)?;
        wtr.write_u128::<Endian>(self.payload_bytes)?;

        assert_eq!(wtr.len(), FILE_HEADER_SIZE);

        Ok(wtr)
    }
}

impl TryFrom<&[u8]> for FileHeader {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert_eq!(bytes.len(), FILE_HEADER_SIZE);

        let mut rdr = Cursor::new(Vec::from(bytes));
        let header_bytes = rdr.read_u128::<Endian>()?;
        let payload_bytes = rdr.read_u128::<Endian>()?;

        Ok(Self {
            header_bytes,
            payload_bytes,
        })
    }
}

impl TryFrom<Vec<u8>> for FileHeader {
    type Error = Error;
    fn try_from(bytes: Vec<u8>) -> result::Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryInto<Vec<u8>> for FileHeader {
    type Error = Error;
    fn try_into(self) -> result::Result<Vec<u8>, Self::Error> {
        self.to_vec_u8()
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
        debug!("creating segment file: {:?}", path.as_ref());

        if path.as_ref().exists() {
            return Err(Error::FileExisted(path.as_ref().to_path_buf()));
        }

        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.as_ref())?;
        let reader = OpenOptions::new().read(true).open(path)?;

        let mut seg_file = Self {
            segment_seq: 0,

            header: FileHeader::new(payload_bytes),
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            segment_bytes: payload_bytes as usize + segment::SEGMENT_HEADER_SIZE,
        };
        trace!("header of new segment file, header={:?}", seg_file.header);

        seg_file.writer.seek(SeekFrom::Start(0))?;
        seg_file
            .writer
            .write_all(seg_file.header.to_vec_u8()?.as_slice())?;
        seg_file
            .reader
            .seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;

        trace!("inited: SegmentFile={:?}", seg_file);
        Ok(seg_file)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        debug!("open segment file: {:?}", path.as_ref());

        let writer = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.as_ref())?;
        let mut reader = OpenOptions::new().read(true).open(path.as_ref())?;

        let mut buf = [0u8; FILE_HEADER_SIZE];
        if let Err(err) = reader.read_exact(&mut buf) {
            match err.kind() {
                io::ErrorKind::UnexpectedEof => {
                    return Err(Error::FileHeaderMissing(path.as_ref().to_path_buf()));
                }
                _ => {
                    return Err(Error::IO(err));
                }
            };
        };

        let header = FileHeader::try_from(&buf[..])?;
        trace!("file-header={:?}", header);
        let segment_bytes = (header.header_bytes + header.payload_bytes) as usize;

        let mut seg_file = Self {
            segment_seq: 0,

            header,
            segment_bytes,
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        };

        seg_file
            .reader
            .seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;
        seg_file.writer.seek(SeekFrom::End(0))?;

        let segments_bytes =
            seg_file.writer.seek(SeekFrom::Current(0))? as usize - FILE_HEADER_SIZE;
        trace!("bytes of segments of file: bytes={}", segments_bytes);
        if segments_bytes == 0 {
            seg_file.segment_seq = 0;
        } else {
            assert_eq!(segments_bytes % seg_file.segment_bytes, 0);
            seg_file.segment_seq = segments_bytes / seg_file.segment_bytes;
        }

        trace!("inited: SegmentFile={:?}", seg_file);
        Ok(seg_file)
    }

    fn next_segment(&mut self) -> Result<Option<Segment>> {
        debug!("reading next segment");
        let mut buf: Vec<u8> = vec![0u8; self.segment_bytes];
        if let Err(err) = self.reader.read_exact(buf.as_mut_slice()) {
            match err.kind() {
                io::ErrorKind::UnexpectedEof => {
                    trace!("read EOF");
                    Ok(None)
                }
                _ => Err(Error::IO(err)),
            }
        } else {
            assert_eq!(buf.len(), self.segment_bytes);
            let segment = Segment::try_from(buf.as_slice())?;
            trace!(
                "read next segment success: segment={{header={:?}, payload.len={}}}",
                segment.header,
                segment.payload.len(),
            );
            Ok(Some(segment))
        }
    }

    // TODO add `next_n_segments(n)->Result<Option<Vec<Segment>>>`

    pub fn last_segment_seq(&self) -> Option<usize> {
        if self.segment_seq == 0 {
            None
        } else {
            Some(self.segment_seq - 1)
        }
    }

    fn write_segments(&mut self, segments: Vec<Segment>) -> Result<()> {
        for seg in segments {
            self.writer.write_all(seg.to_vec_u8()?.as_slice())?;
            self.writer.flush()?;
            self.segment_seq += 1;
        }
        Ok(())
    }

    pub fn seek_segment(&mut self, n: usize) -> Result<Option<u64>> {
        debug!("seeking segment: segment_seq={}", n);

        let bytes = FILE_HEADER_SIZE + self.segment_bytes * n;
        trace!("seek bytes from start: bytes={}", bytes);
        if self.reader.seek(SeekFrom::Start(bytes as u64))? as usize == bytes {
            trace!("segment found");
            Ok(Some(bytes as u64))
        } else {
            trace!("segment not found: encounter EOF");
            Ok(None)
        }
    }

    pub fn get_payload(&mut self) -> Result<Option<Vec<u8>>> {
        debug!("reading payload from segments");

        let mut bytes: Vec<u8> = Vec::with_capacity(self.header.payload_bytes as usize);
        if let Some(seg) = self.next_segment()? {
            if seg.is_first() {
                if seg.is_last() {
                    bytes.extend(seg.payload());
                    trace!(
                        "read success, first contains all: payload.len={}",
                        bytes.len()
                    );
                    return Ok(Some(bytes));
                } else {
                    trace!(
                        "read segments: {}/{}",
                        seg.header.partial_seq + 1,
                        seg.header.total
                    );
                    bytes.extend(seg.payload());
                }
            } else {
                return Err(Error::ReadFromMiddle(
                    seg.header.partial_seq,
                    seg.header.total,
                ));
            }
        } else {
            return Ok(None);
        };
        // TODO use `next_n_segments`
        while let Some(seg) = self.next_segment()? {
            trace!(
                "read segments: {}/{}",
                seg.header.partial_seq + 1,
                seg.header.total
            );
            // TODO: use `take_payload`
            bytes.extend(seg.payload());
            if seg.is_last() {
                trace!("read success: payload.len={}", bytes.len());
                return Ok(Some(bytes));
            }
        }
        panic!("incomplete write")
    }

    pub fn append(&mut self, payload: &[u8]) -> Result<Range<usize>> {
        debug!(
            "appending payload into segment_file: payload.len={}",
            payload.len(),
        );
        let segs = segment::create(payload.to_owned(), self.header.payload_bytes as usize);
        trace!("create {} segments from payload", segs.len());
        let segment_seq = self.segment_seq;
        self.write_segments(segs)?;

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
            let bytes = c.header.to_vec_u8().unwrap();
            assert_eq!(bytes.len(), FILE_HEADER_SIZE);
            let header = FileHeader::try_from(bytes.as_slice()).unwrap();
            assert_eq!(header, c.header);
            assert_eq!(header.to_vec_u8().unwrap(), bytes);
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
        assert!(SegmentFile::create("tmp/tmp.segment", TEST_PAYLOAD_BYTES as u128).is_err());

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
            let bytes_seg = s_file.get_payload().unwrap().unwrap();
            assert_eq!(bytes_seg, bytes_json);

            let c: Case = serde_json::from_slice(bytes_seg.as_slice()).unwrap();
            assert_eq!(&c, case);
        }
        assert!(s_file.get_payload().unwrap().is_none());

        for (i, case) in cases.iter().enumerate() {
            let seek_bytes = s_file.seek_segment(index[&i].start).unwrap().unwrap();
            assert_eq!(
                seek_bytes,
                (FILE_HEADER_SIZE + s_file.segment_bytes * index[&i].start) as u64
            );

            let js_bytes = serde_json::to_vec(case).unwrap();
            let seg_bytes = s_file.get_payload().unwrap().unwrap();
            assert_eq!(seg_bytes, js_bytes);

            let c: Case = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
            assert_eq!(&c, case);
        }
        assert!(s_file.get_payload().unwrap().is_none());

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
            let seg_bytes = s_file.get_payload().unwrap().unwrap();
            assert_eq!(seg_bytes, js_bytes);

            let c: Case = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
            assert_eq!(&c, case);
        }
        assert!(s_file.get_payload().unwrap().is_some());
    }
}
