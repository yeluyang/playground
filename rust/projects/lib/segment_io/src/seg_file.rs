use std::{
    convert::{TryFrom, TryInto},
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write},
    mem,
    ops::Range,
    path::{Path, PathBuf},
    result,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
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

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub write_enable: bool,
}

impl Config {
    fn new<P: AsRef<Path>>(path: P, write_enable: bool) -> Self {
        Self {
            write_enable,
            path: PathBuf::from(path.as_ref()),
        }
    }
}

// TODO ReadOnly and WriteOnly
#[derive(Debug)]
pub struct SegmentsFile {
    pub config: Config,

    header: FileHeader,
    segment_bytes: usize,
    segment_seq: Arc<AtomicUsize>,

    reader: BufReader<File>,
    writer: Option<Arc<Mutex<BufWriter<File>>>>,
}

impl Clone for SegmentsFile {
    fn clone(&self) -> Self {
        let mut reader = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(self.config.path.as_path())
                .unwrap(),
        );
        reader
            .seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))
            .unwrap();
        Self {
            config: self.config.clone(),

            header: self.header.clone(),
            segment_bytes: self.segment_bytes,
            segment_seq: self.segment_seq.clone(),

            reader,
            writer: self.writer.clone(),
        }
    }
}

impl SegmentsFile {
    fn new(config: Config, header: FileHeader, segment_bytes: usize) -> Result<Self> {
        let reader = BufReader::new(OpenOptions::new().read(true).open(config.path.as_path())?);
        let writer = if config.write_enable {
            Some(Arc::new(Mutex::new(BufWriter::new(
                OpenOptions::new().write(true).open(config.path.as_path())?,
            ))))
        } else {
            None
        };

        Ok(Self {
            config,
            header,
            segment_bytes,
            segment_seq: Arc::new(AtomicUsize::new(0)),

            reader,
            writer,
        })
    }

    pub fn create<P: AsRef<Path>>(path: P, payload_bytes: u128) -> Result<Self> {
        debug!("creating segment file: {:?}", path.as_ref());

        if path.as_ref().exists() {
            return Err(Error::FileExisted(path.as_ref().to_path_buf()));
        } else {
            File::create(path.as_ref())?;
        }

        if payload_bytes == 0 {
            return Err(Error::PayloadLimitZero);
        }

        let mut seg_file = Self::new(
            Config::new(path, true),
            FileHeader::new(payload_bytes),
            payload_bytes as usize + segment::SEGMENT_HEADER_SIZE,
        )?;
        trace!("header of new segment file, config={:?}", seg_file.config);

        {
            let mut seg_writer = seg_file.writer.as_ref().unwrap().lock().unwrap();
            seg_writer.seek(SeekFrom::Start(0))?;
            seg_writer.write_all(seg_file.header.to_vec_u8()?.as_slice())?;
        }

        seg_file
            .reader
            .seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;

        trace!("inited: SegmentsFile={:?}", seg_file);
        Ok(seg_file)
    }

    pub fn open<P: AsRef<Path>>(path: P, write_enable: bool) -> Result<Self> {
        debug!("open segment file: {:?}", path.as_ref());

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
        drop(reader);

        let mut seg_file = Self::new(
            Config::new(path.as_ref(), write_enable),
            header,
            segment_bytes,
        )?;

        let segments_bytes = seg_file.reader.seek(SeekFrom::End(0))? as usize - FILE_HEADER_SIZE;
        seg_file
            .reader
            .seek(SeekFrom::Start(FILE_HEADER_SIZE as u64))?;

        trace!("bytes of segments of file: bytes={}", segments_bytes);
        if segments_bytes == 0 {
            seg_file.segment_seq.store(0, Ordering::SeqCst);
        } else {
            assert_eq!(segments_bytes % seg_file.segment_bytes, 0);
            seg_file
                .segment_seq
                .store(segments_bytes / seg_file.segment_bytes, Ordering::SeqCst);
        }

        trace!("inited: SegmentsFile={:?}", seg_file);
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
        if self.segment_seq.load(Ordering::SeqCst) == 0 {
            None
        } else {
            Some(self.segment_seq.load(Ordering::SeqCst) - 1)
        }
    }

    fn write_segments(&mut self, segments: Vec<Segment>) -> Result<()> {
        match &self.writer {
            None => Err(Error::WriteOnReadOnlyFile(self.config.path.clone())),
            Some(writer) => {
                let mut writer = writer.lock().unwrap();
                for seg in segments {
                    writer.write_all(seg.to_vec_u8()?.as_slice())?;
                    writer.flush()?;
                    self.segment_seq.fetch_add(1, Ordering::SeqCst);
                }
                Ok(())
            }
        }
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

    pub fn next_payload(&mut self) -> Result<Option<Vec<u8>>> {
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

    pub fn read_payload_by(&mut self, n: usize) -> Result<Option<Vec<u8>>> {
        match self.next_payload() {
            Ok(opt) => Ok(opt),
            Err(err) => match err {
                Error::ReadFromMiddle(seq, _) => {
                    self.seek_segment(n - seq as usize)?;
                    self.next_payload()
                }
                _ => Err(err),
            },
        }
    }

    pub fn append(&mut self, payload: &[u8]) -> Result<Range<usize>> {
        debug!(
            "appending payload into segment_file: payload.len={}",
            payload.len(),
        );
        let segs = segment::create(payload.to_owned(), self.header.payload_bytes as usize);
        trace!("create {} segments from payload", segs.len());
        let segment_seq = self.segment_seq.load(Ordering::SeqCst);
        self.write_segments(segs)?;

        trace!(
            "write all success: segment_file.segment_seq={{before={}, now={}}}",
            segment_seq,
            self.segment_seq.load(Ordering::SeqCst)
        );
        Ok(Range {
            start: segment_seq,
            end: self.segment_seq.load(Ordering::SeqCst),
        })
    }

    // TODO add `replace`
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    mod file_header {
        use super::*;

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
    }

    mod segments_file {
        use std::{collections::HashMap, fs};

        use super::*;

        extern crate serde;
        use serde::{Deserialize, Serialize};
        extern crate serde_json;

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct CaseData {
            id: usize,
            data: Vec<u8>,
        }

        fn setup<T, P>(
            dataset: &[T],
            path: P,
            payload_limits: usize,
        ) -> (SegmentsFile, HashMap<usize, Range<usize>>)
        where
            T: Serialize,
            P: AsRef<Path>,
        {
            let mut index: HashMap<usize, Range<usize>> = HashMap::new();
            let mut s_file = SegmentsFile::create(path, payload_limits as u128).unwrap();

            for (i, data) in dataset.iter().enumerate() {
                let bytes = serde_json::to_vec(data).unwrap();
                let seq_rng = s_file.append(bytes.as_slice()).unwrap();
                index.insert(i, seq_rng);
            }
            assert_eq!(index.len(), dataset.len());
            (s_file, index)
        }

        #[test]
        fn test_create() {
            init();
            let case_dir = case_dir(module_path!(), "test_create");
            if case_dir.exists() {
                fs::remove_dir_all(&case_dir).unwrap();
            }
            fs::create_dir_all(&case_dir).unwrap();

            struct Case {
                path: String,
                payload_limits: u128,
                result: Result<()>,
            }
            let cases = &[
                Case {
                    path: "normal.segment".to_owned(),
                    payload_limits: 128,
                    result: Ok(()),
                },
                Case {
                    path: "non-existed-dir/non-existed.segment".to_owned(),
                    payload_limits: 128,
                    result: Err(Error::IO(io::Error::from_raw_os_error(2))),
                },
                Case {
                    path: "payload-limits-zero.segment".to_owned(),
                    payload_limits: 0,
                    result: Err(Error::PayloadLimitZero),
                },
            ];

            for case in cases {
                let path = case_dir.join(&case.path);
                match SegmentsFile::create(&path, case.payload_limits) {
                    Ok(s_file) => {
                        assert_eq!(s_file.segment_seq.load(Ordering::SeqCst), 0);
                        assert_eq!(
                            s_file.segment_bytes,
                            segment::SEGMENT_HEADER_SIZE + case.payload_limits as usize
                        );
                        assert_eq!(
                            s_file.header.header_bytes as usize,
                            segment::SEGMENT_HEADER_SIZE
                        );
                        assert_eq!(s_file.header.payload_bytes, case.payload_limits);
                        assert!(SegmentsFile::create(&path, case.payload_limits).is_err());
                    }
                    Err(err) => assert_eq!(
                        err.to_string(),
                        case.result.as_ref().unwrap_err().to_string()
                    ),
                };
            }
        }

        #[test]
        fn test_open() {
            init();
            let case_dir = case_dir(module_path!(), "test_open");
            if case_dir.exists() {
                fs::remove_dir_all(&case_dir).unwrap();
            }
            fs::create_dir_all(&case_dir).unwrap();

            struct Case {
                path: String,
                payload_limits: usize,
                dataset: Vec<CaseData>,
                result: Result<()>,
            }
            let cases = &[
                Case {
                    path: "normal.segment".to_owned(),
                    payload_limits: 128,
                    dataset: vec![
                        CaseData {
                            id: 0,
                            data: vec![0; 64],
                        },
                        CaseData {
                            id: 1,
                            data: vec![1; 256],
                        },
                    ],
                    result: Ok(()),
                },
                Case {
                    path: "no-header.segment".to_owned(),
                    payload_limits: 128,
                    dataset: vec![],
                    result: Err(Error::FileHeaderMissing(case_dir.join("no-header.segment"))),
                },
            ];

            for case in cases {
                let path = case_dir.join(&case.path);
                let err = SegmentsFile::open(&path, false).unwrap_err();
                assert_eq!(
                    err.to_string(),
                    Error::IO(io::Error::from_raw_os_error(2)).to_string()
                );

                let (open_result, index) = match case.result.as_ref() {
                    Ok(_) => {
                        let (_, index) = setup(case.dataset.as_slice(), &path, case.payload_limits);

                        (SegmentsFile::open(&path, false), index)
                    }
                    Err(err) => match err {
                        Error::FileHeaderMissing(_) => {
                            OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open(&path)
                                .unwrap();
                            (SegmentsFile::open(&path, false), HashMap::new())
                        }
                        _ => unreachable!(),
                    },
                };

                match open_result {
                    Ok(s_file) => {
                        assert_eq!(
                            s_file.segment_seq.load(Ordering::SeqCst),
                            index[&(case.dataset.len() - 1)].end
                        );
                        assert_eq!(
                            s_file.segment_bytes,
                            segment::SEGMENT_HEADER_SIZE + case.payload_limits
                        );
                        assert_eq!(
                            s_file.header.header_bytes as usize,
                            segment::SEGMENT_HEADER_SIZE
                        );
                        assert_eq!(s_file.header.payload_bytes as usize, case.payload_limits);
                    }
                    Err(err) => {
                        assert_eq!(
                            err.to_string(),
                            case.result.as_ref().unwrap_err().to_string(),
                        );
                    }
                }
            }
        }

        #[test]
        fn test_read() {
            init();
            let case_dir = case_dir(module_path!(), "test_read");
            if case_dir.exists() {
                fs::remove_dir_all(&case_dir).unwrap();
            }
            fs::create_dir_all(&case_dir).unwrap();

            struct Case {
                path: String,
                payload_limits: usize,
                dataset: Vec<CaseData>,
            }
            let cases = &[Case {
                path: "normal.segment".to_owned(),
                payload_limits: 128,
                dataset: vec![
                    CaseData {
                        id: 0,
                        data: vec![0; 64],
                    },
                    CaseData {
                        id: 1,
                        data: vec![1; 256],
                    },
                ],
            }];

            for case in cases {
                let path = case_dir.join(&case.path);
                let (mut s_file, _) = setup(case.dataset.as_slice(), &path, case.payload_limits);
                for i in 0..2 {
                    for data in &case.dataset {
                        let js_bytes = serde_json::to_vec(data).unwrap();
                        let seg_bytes = s_file.next_payload().unwrap().unwrap();
                        assert_eq!(seg_bytes, js_bytes);

                        let d: CaseData = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
                        assert_eq!(&d, data);
                    }
                    assert!(s_file.next_payload().unwrap().is_none());

                    if i == 0 {
                        // open then read
                        s_file = SegmentsFile::open(&path, false).unwrap();
                    }
                }
            }
        }

        #[test]
        fn test_seek() {
            init();
            let case_dir = case_dir(module_path!(), "test_seek");
            if case_dir.exists() {
                fs::remove_dir_all(&case_dir).unwrap();
            }
            fs::create_dir_all(&case_dir).unwrap();

            struct Case {
                path: String,
                payload_limits: usize,
                dataset: Vec<CaseData>,
            }
            let cases = &[Case {
                path: "normal.segment".to_owned(),
                payload_limits: 128,
                dataset: vec![
                    CaseData {
                        id: 0,
                        data: vec![0; 64],
                    },
                    CaseData {
                        id: 1,
                        data: vec![1; 256],
                    },
                ],
            }];

            for case in cases {
                let path = case_dir.join(&case.path);
                let (mut s_file, index) =
                    setup(case.dataset.as_slice(), &path, case.payload_limits);
                for i in 0..2 {
                    for (i, data) in case.dataset.iter().rev().enumerate() {
                        let seek_bytes = s_file
                            .seek_segment(index[&(case.dataset.len() - i - 1)].start)
                            .unwrap()
                            .unwrap();
                        assert_eq!(
                            seek_bytes,
                            (FILE_HEADER_SIZE
                                + s_file.segment_bytes * index[&(case.dataset.len() - i - 1)].start)
                                as u64
                        );
                        let js_bytes = serde_json::to_vec(data).unwrap();
                        let seg_bytes = s_file.next_payload().unwrap().unwrap();
                        assert_eq!(seg_bytes, js_bytes);
                        let d: CaseData = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
                        assert_eq!(&d, data);
                    }
                    assert!(s_file.next_payload().unwrap().is_some());

                    if i == 0 {
                        // open then seek
                        s_file = SegmentsFile::open(&path, false).unwrap();
                    }
                }
            }
        }

        #[test]
        fn test_concurrence() {
            // TODO add concurrence test
        }
    }
}
