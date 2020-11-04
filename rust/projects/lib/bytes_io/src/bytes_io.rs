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
    entry::{EntryID, EntryOffset},
    error::{Error, Result},
    frame::{self, Frame, Header},
    Endian, Version, CURRENT_VERSION, VERSION_BYTES,
};

#[derive(Debug, Clone, Default, PartialEq)]
struct Meta {
    version: Version,
    uuid: u128,
    header_bytes: u128,
    payload_bytes: u128,
}
const META_BYTES: usize = mem::size_of::<Meta>();

impl Meta {
    fn new(payload_bytes: u128) -> Self {
        Self {
            version: Version::new(),
            uuid: 0, // TODO
            header_bytes: frame::HEADER_SIZE as u128,
            payload_bytes,
        }
    }

    /// length of frame in file, unit=bytes
    fn frame_len(&self) -> usize {
        self.header_bytes as usize + self.payload_bytes as usize
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = self.version.to_bytes()?;
        assert_eq!(bytes.len(), VERSION_BYTES);

        bytes.write_u128::<Endian>(self.uuid)?;
        bytes.write_u128::<Endian>(self.header_bytes)?;
        bytes.write_u128::<Endian>(self.payload_bytes)?;

        assert_eq!(bytes.len(), META_BYTES);

        Ok(bytes)
    }
}

impl TryFrom<&[u8]> for Meta {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert_eq!(bytes.len(), META_BYTES);

        let version = Version::try_from(&bytes[..VERSION_BYTES])?;
        if !version.is_compatible() {
            return Err(Error::Incompatible(CURRENT_VERSION, version));
        }
        let mut r = Cursor::new(Vec::from(&bytes[VERSION_BYTES..]));
        let uuid = r.read_u128::<Endian>()?;
        let header_bytes = r.read_u128::<Endian>()?;
        let payload_bytes = r.read_u128::<Endian>()?;

        Ok(Self {
            version,
            uuid,
            header_bytes,
            payload_bytes,
        })
    }
}

impl TryFrom<Vec<u8>> for Meta {
    type Error = Error;
    fn try_from(bytes: Vec<u8>) -> result::Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryInto<Vec<u8>> for Meta {
    type Error = Error;
    fn try_into(self) -> result::Result<Vec<u8>, Self::Error> {
        self.to_bytes()
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
pub struct BytesIO {
    pub config: Config,

    meta: Meta,
    entry_current_seq: u128,
    frame_offset: Arc<AtomicUsize>, // offset for frames in file

    reader: BufReader<File>,
    writer: Option<Arc<Mutex<BufWriter<File>>>>,
}

impl Clone for BytesIO {
    fn clone(&self) -> Self {
        let mut reader = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(self.config.path.as_path())
                .unwrap(),
        );
        reader.seek(SeekFrom::Start(META_BYTES as u64)).unwrap();
        Self {
            config: self.config.clone(),

            meta: self.meta.clone(),
            entry_current_seq: self.entry_current_seq,
            frame_offset: self.frame_offset.clone(),

            reader,
            writer: self.writer.clone(),
        }
    }
}

impl BytesIO {
    fn new(config: Config, meta: Meta) -> Result<Self> {
        trace!("new BytesIO with: config={:?}, meta={:?}", config, meta);
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
            meta,
            entry_current_seq: 0, // TODO
            frame_offset: Arc::new(AtomicUsize::new(0)),

            reader,
            writer,
        })
    }

    pub fn create<P: AsRef<Path>>(path: P, payload_bytes: u128) -> Result<Self> {
        trace!(
            "creating BytesIO file: on {:?}, with {} Bytes payload",
            path.as_ref(),
            payload_bytes
        );

        if path.as_ref().exists() {
            return Err(Error::FileExisted(path.as_ref().to_path_buf()));
        } else {
            File::create(path.as_ref())?;
        }

        if payload_bytes == 0 {
            return Err(Error::PayloadLimitZero);
        }

        let mut file = Self::new(Config::new(path, true), Meta::new(payload_bytes))?;

        {
            let mut writer = file.writer.as_ref().unwrap().lock().unwrap();
            writer.seek(SeekFrom::Start(0))?;
            writer.write_all(file.meta.to_bytes()?.as_slice())?;
        }

        file.reader.seek(SeekFrom::Start(META_BYTES as u64))?;

        Ok(file)
    }

    pub fn open<P: AsRef<Path>>(path: P, write_enable: bool) -> Result<Self> {
        trace!(
            "open BytesIO file: {:?}, with write_permission={}",
            path.as_ref(),
            write_enable
        );

        let mut reader = OpenOptions::new().read(true).open(path.as_ref())?;
        let mut buf = [0u8; META_BYTES];
        if let Err(err) = reader.read_exact(&mut buf) {
            match err.kind() {
                io::ErrorKind::UnexpectedEof => {
                    return Err(Error::MetaMissing(path.as_ref().to_path_buf()));
                }
                _ => {
                    return Err(Error::IO(err));
                }
            };
        };
        let meta = Meta::try_from(&buf[..])?;
        debug!("read meta from BytesIO file existed: meta={:?}", meta);
        drop(reader);

        let mut file = Self::new(Config::new(path.as_ref(), write_enable), meta)?;

        let frame_bytes_existed = file.reader.seek(SeekFrom::End(0))? as usize - META_BYTES;
        debug!("{} bytes of frames exists in file", frame_bytes_existed);
        if frame_bytes_existed == 0 {
            file.frame_offset.store(0, Ordering::SeqCst);
        } else {
            assert_eq!(frame_bytes_existed % file.meta.frame_len(), 0);
            file.frame_offset.store(
                frame_bytes_existed / file.meta.frame_len(),
                Ordering::SeqCst,
            );
        }

        file.reader
            .seek(SeekFrom::End(-1 * file.meta.frame_len() as i64));
        file.entry_current_seq = if let Some(header_last) = file.read_header()? {
            header_last.entry_seq + 1
        } else {
            0
        };

        file.reader.seek(SeekFrom::Start(META_BYTES as u64))?;

        Ok(file)
    }

    pub fn read_entry(&mut self) -> Result<Option<Vec<u8>>> {
        trace!("reading entry");

        if let Some(frame_first) = self.read_frame()? {
            if frame_first.is_first() {
                let mut frame_count = 0u128;
                let mut bytes: Vec<u8> = Vec::with_capacity(
                    self.meta.payload_bytes as usize * frame_first.header.total as usize,
                );
                bytes.extend(frame_first.payload());
                for _ in 0..frame_first.header.total - 1 {
                    if let Some(frame) = self.read_frame()? {
                        trace!(
                            "read a frame({}/{}) from an entry: header={:?}",
                            frame.header.frame_seq + 1,
                            frame_first.header.total,
                            frame.header
                        );
                        assert_eq!(frame.header.entry_seq, frame_first.header.entry_seq,);
                        assert_eq!(frame.header.frame_seq, frame_count,);
                        bytes.extend(frame.payload());
                        frame_count += 1;
                    } else {
                        return Err(Error::MeetIncompleteEntry(
                            frame_first.header.total,
                            frame_count,
                        ));
                    }
                }
                debug!(
                    "read entry: frames={}, sequence={}",
                    frame_count, frame_first.header.entry_seq
                );
                return Ok(Some(bytes));
            } else {
                // XXX: allow read from middle of entry?
                panic!(
                    "read from middle of entry: {} in {}",
                    frame_first.header.frame_seq + 1,
                    frame_first.header.total
                );
            }
        } else {
            return Ok(None);
        };
    }

    pub fn append(&mut self, payload: &[u8]) -> Result<EntryOffset> {
        trace!("writing {} bytes into BytesIO file", payload.len(),);
        let frames = frame::create(
            payload.to_owned(),
            self.entry_current_seq,
            self.meta.payload_bytes as usize,
        );
        let frames_num = frames.len();
        let first_frame = self.frame_offset.load(Ordering::SeqCst);
        let entry_seq = self.entry_current_seq;
        self.write_frames(frames)?;
        let offset_current = self.frame_offset.load(Ordering::SeqCst);
        assert_eq!(offset_current - first_frame, frames_num);

        debug!(
            "write success, offset of frames: {} -> {})",
            first_frame, offset_current
        );
        Ok(EntryOffset::new(self.meta.uuid, entry_seq, first_frame))
    }

    pub fn seek_entry(&mut self, offset: EntryOffset) -> Result<Option<()>> {
        trace!("seek entry on offset={:?}", offset);

        if self.meta.uuid != offset.entry_id.file_id {
            Ok(None)
        } else if let Some(header) = self.seek_frame(offset.first_frame)? {
            if header.entry_seq != offset.entry_id.entry_seq {
                Err(Error::EntryMismatch(
                    offset.entry_id.entry_seq,
                    header.entry_seq,
                ))
            } else {
                Ok(Some(()))
            }
        } else {
            Ok(None)
        }
    }

    /// TODO
    pub fn first_entry(&mut self) -> Result<Option<EntryOffset>> {
        unimplemented!()
    }

    /// TODO
    pub fn last_entry(&mut self) -> Result<Option<EntryOffset>> {
        unimplemented!()
    }

    /// TODO
    pub fn find_entry(&mut self, entry_id: EntryID) -> Result<EntryOffset> {
        unimplemented!()
    }

    // TODO add `replace`, need `entry::Reserve`

    fn read_frame(&mut self) -> Result<Option<Frame>> {
        trace!("reading next frame");
        if let Some(buf) = self.read_into(self.meta.frame_len() as usize)? {
            let frame = Frame::try_from(buf.as_slice())?;
            debug!(
                "read next frame success: frame={{header={:?}, payload.len={}}}",
                frame.header,
                frame.payload().len(),
            );
            Ok(Some(frame))
        } else {
            debug!("next frame not found");
            Ok(None)
        }
    }

    /// TODO
    fn read_batch_frames(batch: usize) -> Result<Option<Vec<Frame>>> {
        unimplemented!()
    }

    fn read_header(&mut self) -> Result<Option<Header>> {
        trace!("reading next header of frame");
        if let Some(buf) = self.read_into(self.meta.header_bytes as usize)? {
            let header = Header::try_from(buf.as_slice())?;
            debug!("read next header of frame success: header={:?}", header,);
            Ok(Some(header))
        } else {
            debug!("next header of frame not found");
            Ok(None)
        }
    }

    fn read_into(&mut self, bytes: usize) -> Result<Option<Vec<u8>>> {
        trace!("reading {} bytes from BytesIO file", bytes);
        let mut buf: Vec<u8> = vec![0u8; bytes];
        if let Err(err) = self.reader.read_exact(buf.as_mut_slice()) {
            match err.kind() {
                io::ErrorKind::UnexpectedEof => Ok(None),
                _ => Err(Error::IO(err)),
            }
        } else {
            Ok(Some(buf))
        }
    }

    fn write_frames(&mut self, frames: Vec<Frame>) -> Result<()> {
        match &self.writer {
            None => Err(Error::WriteOnReadOnlyFile(self.config.path.clone())),
            Some(writer) => {
                let mut writer = writer.lock().unwrap();
                for frame in frames {
                    writer.write_all(frame.to_bytes()?.as_slice())?;
                    writer.flush()?;
                    self.frame_offset.fetch_add(1, Ordering::SeqCst);
                }
                Ok(())
            }
        }
    }

    fn seek_frame(&mut self, n: usize) -> Result<Option<Header>> {
        trace!("seeking frame on offset {}", n);

        let bytes = META_BYTES + self.meta.frame_len() * n;
        let offset = SeekFrom::Start(bytes as u64);
        if self.reader.seek(offset)? as usize == bytes {
            let result = self.read_header();
            self.reader.seek(offset)?;
            return result;
        } else {
            trace!("frame seeking not found");
            self.reader.seek(SeekFrom::End(0))?;
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    mod meta {
        use super::*;

        #[test]
        fn test_file_header() {
            init();
            struct Case {
                header: Meta,
            }
            let cases = [Case {
                header: Meta::new(128),
            }];

            for c in cases.iter() {
                let bytes = c.header.to_bytes().unwrap();
                assert_eq!(bytes.len(), META_BYTES);
                let header = Meta::try_from(bytes.as_slice()).unwrap();
                assert_eq!(header, c.header);
                assert_eq!(header.to_bytes().unwrap(), bytes);
            }
        }
    }

    mod bytes_io {
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
        ) -> (BytesIO, HashMap<usize, EntryOffset>)
        where
            T: Serialize,
            P: AsRef<Path>,
        {
            let mut index: HashMap<usize, EntryOffset> = HashMap::new();
            let mut s_file = BytesIO::create(path, payload_limits as u128).unwrap();

            for (i, data) in dataset.iter().enumerate() {
                let bytes = serde_json::to_vec(data).unwrap();
                let entry_offset = s_file.append(bytes.as_slice()).unwrap();
                index.insert(i, entry_offset);
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
                    path: "normal.frame".to_owned(),
                    payload_limits: 128,
                    result: Ok(()),
                },
                Case {
                    path: "non-existed-dir/non-existed.frame".to_owned(),
                    payload_limits: 128,
                    result: Err(Error::IO(io::Error::from_raw_os_error(2))),
                },
                Case {
                    path: "payload-limits-zero.frame".to_owned(),
                    payload_limits: 0,
                    result: Err(Error::PayloadLimitZero),
                },
            ];

            for case in cases {
                let path = case_dir.join(&case.path);
                match BytesIO::create(&path, case.payload_limits) {
                    Ok(s_file) => {
                        assert_eq!(s_file.frame_offset.load(Ordering::SeqCst), 0);
                        assert_eq!(
                            s_file.meta.frame_len(),
                            frame::HEADER_SIZE + case.payload_limits as usize
                        );
                        assert_eq!(s_file.meta.header_bytes as usize, frame::HEADER_SIZE);
                        assert_eq!(s_file.meta.payload_bytes, case.payload_limits);
                        assert!(BytesIO::create(&path, case.payload_limits).is_err());
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
                    path: "normal.frame".to_owned(),
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
                    path: "no-header.frame".to_owned(),
                    payload_limits: 128,
                    dataset: vec![],
                    result: Err(Error::MetaMissing(case_dir.join("no-header.frame"))),
                },
            ];

            for case in cases {
                let path = case_dir.join(&case.path);
                let err = BytesIO::open(&path, false).unwrap_err();
                assert_eq!(
                    err.to_string(),
                    Error::IO(io::Error::from_raw_os_error(2)).to_string()
                );

                let (open_result, index) = match case.result.as_ref() {
                    Ok(_) => {
                        let (_, index) = setup(case.dataset.as_slice(), &path, case.payload_limits);

                        (BytesIO::open(&path, false), index)
                    }
                    Err(err) => match err {
                        Error::MetaMissing(_) => {
                            OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open(&path)
                                .unwrap();
                            (BytesIO::open(&path, false), HashMap::new())
                        }
                        _ => unreachable!(),
                    },
                };

                match open_result {
                    Ok(s_file) => {
                        assert_eq!(
                            s_file.frame_offset.load(Ordering::SeqCst),
                            index[&(case.dataset.len() - 1)].first_frame,
                        );
                        assert_eq!(
                            s_file.meta.frame_len(),
                            frame::HEADER_SIZE + case.payload_limits
                        );
                        assert_eq!(s_file.meta.header_bytes as usize, frame::HEADER_SIZE);
                        assert_eq!(s_file.meta.payload_bytes as usize, case.payload_limits);
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
                path: "normal.frame".to_owned(),
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
                        let seg_bytes = s_file.read_entry().unwrap().unwrap();
                        assert_eq!(seg_bytes, js_bytes);

                        let d: CaseData = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
                        assert_eq!(&d, data);
                    }
                    assert!(s_file.read_entry().unwrap().is_none());

                    if i == 0 {
                        // open then read
                        s_file = BytesIO::open(&path, false).unwrap();
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
                path: "normal.frame".to_owned(),
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
                        // FIXME: syntax error when `seek_frame` modified
                        // let seek_bytes = s_file
                        //     .seek_frame(index[&(case.dataset.len() - i - 1)].start)
                        //     .unwrap()
                        //     .unwrap();
                        // assert_eq!(
                        //     seek_bytes,
                        //     (META_BYTES
                        //         + s_file.meta.frame_len()
                        //             * index[&(case.dataset.len() - i - 1)].start)
                        //         as u64
                        // );
                        let js_bytes = serde_json::to_vec(data).unwrap();
                        let seg_bytes = s_file.read_entry().unwrap().unwrap();
                        assert_eq!(seg_bytes, js_bytes);
                        let d: CaseData = serde_json::from_slice(seg_bytes.as_slice()).unwrap();
                        assert_eq!(&d, data);
                    }
                    assert!(s_file.read_entry().unwrap().is_some());

                    if i == 0 {
                        // open then seek
                        s_file = BytesIO::open(&path, false).unwrap();
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
