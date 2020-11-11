use std::{
    convert::{TryFrom, TryInto},
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write},
    mem,
    path::{Path, PathBuf},
    result,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

extern crate byteorder;
use byteorder::{ReadBytesExt, WriteBytesExt};

extern crate uuid;

use crate::{
    common::{EntryID, EntryOffset, Version, CURRENT_VERSION, VERSION_BYTES},
    error::{Error, Result},
    frame::{self, Frame, Header},
    Endian,
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
        assert!(payload_bytes > 0);
        Self {
            version: Version::new(),
            uuid: uuid::Uuid::new_v4().as_u128(),
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
    reader: BufReader<File>,

    // XXX: should package following members into one mutex?
    entry_next_seq: u128,                // XXX: should wrap by atomic?
    frame_next_offset: Arc<AtomicUsize>, // offset for frames in file
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
            entry_next_seq: self.entry_next_seq,
            frame_next_offset: self.frame_next_offset.clone(),

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
            entry_next_seq: 0,
            frame_next_offset: Arc::new(AtomicUsize::new(0)),

            reader,
            writer,
        })
    }

    pub fn create<P: AsRef<Path>>(path: P, payload_bytes: u128) -> Result<Self> {
        assert!(payload_bytes > 0);
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

        let mut file = Self::new(Config::new(path, true), Meta::new(payload_bytes))?;

        {
            let mut writer = file.writer.as_ref().unwrap().lock().unwrap();
            writer.seek(SeekFrom::Start(0))?;
            writer.write_all(file.meta.to_bytes()?.as_slice())?;
            writer.flush()?;
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
        if !meta.version.is_compatible() {
            return Err(Error::Incompatible(CURRENT_VERSION, meta.version));
        };
        drop(reader);

        let mut file = Self::new(Config::new(path.as_ref(), write_enable), meta)?;

        let frame_bytes_existed = file.reader.seek(SeekFrom::End(0))? as usize - META_BYTES;
        debug!("{} bytes of frames exists in file", frame_bytes_existed);
        if frame_bytes_existed == 0 {
            file.frame_next_offset.store(0, Ordering::SeqCst);
            file.entry_next_seq = 0;
        } else {
            assert_eq!(frame_bytes_existed % file.meta.frame_len(), 0);
            file.frame_next_offset.store(
                frame_bytes_existed / file.meta.frame_len(),
                Ordering::SeqCst,
            );
            file.reader
                .seek(SeekFrom::End(-1 * file.meta.frame_len() as i64))?;
            file.entry_next_seq = file.read_header()?.unwrap().entry_seq + 1;
        }

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
                        frame_count += 1;
                        trace!(
                            "read a frame({}/{}) from an entry: header={:?}",
                            frame.header.frame_seq + 1,
                            frame_first.header.total,
                            frame.header
                        );
                        assert_eq!(frame.header.entry_seq, frame_first.header.entry_seq,);
                        assert_eq!(frame.header.frame_seq, frame_count,);
                        bytes.extend(frame.payload());
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
        match &self.writer {
            None => Err(Error::WriteOnReadOnlyFile(self.config.path.clone())),
            Some(writer) => {
                let frames = frame::create(payload.to_owned(), 0, self.meta.payload_bytes as usize);

                let mut writer = writer.lock().unwrap();

                let entry_seq = self.entry_next_seq;
                self.entry_next_seq += 1;
                let first_frame = self.frame_next_offset.load(Ordering::SeqCst);

                let frames_num = frames.len();
                for mut frame in frames {
                    frame.header.entry_seq = entry_seq;
                    writer.write_all(frame.to_bytes()?.as_slice())?;
                    writer.flush()?;
                    self.frame_next_offset.fetch_add(1, Ordering::SeqCst);
                }

                let offset_current = self.frame_next_offset.load(Ordering::SeqCst);
                writer.flush()?;

                assert_eq!(offset_current - first_frame, frames_num);
                debug!(
                    "write success, offset of frames: {} -> {})",
                    first_frame, offset_current
                );

                Ok(EntryOffset::new(self.meta.uuid, entry_seq, first_frame))
            }
        }
    }

    pub fn seek_entry(&mut self, offset: &EntryOffset) -> Result<Option<()>> {
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
        trace!("reading {} bytes from {:?}", bytes, self.config.path);
        let mut buf: Vec<u8> = vec![0u8; bytes];
        if let Err(err) = self.reader.read_exact(buf.as_mut_slice()) {
            match err.kind() {
                io::ErrorKind::UnexpectedEof => {
                    debug!("encounter EOF of {:?}", self.config.path);
                    Ok(None)
                }
                _ => Err(Error::IO(err)),
            }
        } else {
            debug!(
                "read {}/{} bytes from {:?}",
                buf.len(),
                bytes,
                self.config.path
            );
            Ok(Some(buf))
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
        fn test_meta() {
            init();
            struct Case {
                meta: Meta,
            }
            let cases = [Case {
                meta: Meta::new(128),
            }];

            for c in cases.iter() {
                let bytes = c.meta.to_bytes().unwrap();
                assert_ne!(c.meta.uuid, 0);
                assert_eq!(bytes.len(), META_BYTES);

                let meta = Meta::try_from(bytes.as_slice()).unwrap();
                assert_eq!(meta, c.meta);
                assert_eq!(meta.to_bytes().unwrap(), bytes);
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
            payload_limits: u128,
        ) -> (BytesIO, HashMap<usize, EntryOffset>)
        where
            T: Serialize,
            P: AsRef<Path>,
        {
            let mut index: HashMap<usize, EntryOffset> = HashMap::new();
            let mut file = BytesIO::create(path, payload_limits as u128).unwrap();

            for (i, data) in dataset.iter().enumerate() {
                let bytes = serde_json::to_vec(data).unwrap();
                let entry_offset = file.append(bytes.as_slice()).unwrap();
                index.insert(i, entry_offset);
            }
            assert_eq!(index.len(), dataset.len());
            (file, index)
        }

        #[test]
        fn test_create() {
            init();
            let case_dir = make_clean_case_dir(module_path!(), "test_create");

            struct Case {
                path: String,
                payload_limits: u128,
                result: Result<()>,
            }
            let cases = &[
                Case {
                    path: "normal.bytesio".to_owned(),
                    payload_limits: 128,
                    result: Ok(()),
                },
                Case {
                    path: "non-existed-dir/non-existed.bytesio".to_owned(),
                    payload_limits: 128,
                    result: Err(Error::IO(io::Error::from_raw_os_error(2))),
                },
            ];

            for c in cases {
                let path = case_dir.join(&c.path);
                match BytesIO::create(&path, c.payload_limits) {
                    Ok(mut file) => {
                        assert_eq!(file.frame_next_offset.load(Ordering::SeqCst), 0);
                        assert_eq!(
                            file.meta.frame_len(),
                            frame::HEADER_SIZE + c.payload_limits as usize
                        );
                        assert_eq!(file.meta.header_bytes as usize, frame::HEADER_SIZE);
                        assert_eq!(file.meta.payload_bytes, c.payload_limits);
                        assert!(BytesIO::create(&path, c.payload_limits).is_err());

                        file.reader.seek(SeekFrom::Start(0)).unwrap();
                        let mut buf = vec![0; META_BYTES];
                        file.reader.read_exact(buf.as_mut_slice()).unwrap();
                        let meta = Meta::try_from(buf.as_slice()).unwrap();
                        assert_eq!(meta, file.meta);
                        assert_eq!(file.reader.read(buf.as_mut_slice()).unwrap(), 0);
                    }
                    Err(err) => {
                        assert_eq!(err.to_string(), c.result.as_ref().unwrap_err().to_string())
                    }
                };
            }
        }

        #[test]
        fn test_open() {
            init();
            let case_dir = make_clean_case_dir(module_path!(), "test_open");

            struct Case {
                path: String,
                payload_limits: u128,
                dataset: Vec<CaseData>,
            }
            let cases = &[
                Case {
                    path: "empty.bytesio".to_owned(),
                    payload_limits: 128,
                    dataset: vec![],
                },
                Case {
                    path: "normal.bytesio".to_owned(),
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
                },
            ];

            for c in cases {
                let path = case_dir.join(&c.path);
                setup(c.dataset.as_slice(), &path, c.payload_limits);

                let file = BytesIO::open(&path, false).unwrap();
                assert_eq!(
                    file.meta.frame_len(),
                    frame::HEADER_SIZE + c.payload_limits as usize
                );
                assert_eq!(file.meta.header_bytes as usize, frame::HEADER_SIZE);
                assert_eq!(file.meta.payload_bytes, c.payload_limits);
                assert_eq!(file.entry_next_seq as usize, c.dataset.len());
            }
        }

        #[test]
        fn test_read() {
            init();
            let case_dir = make_clean_case_dir(module_path!(), "test_read");

            struct Case {
                path: String,
                payload_limits: u128,
                dataset: Vec<CaseData>,
            }
            let cases = &[Case {
                path: "normal.bytesio".to_owned(),
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
                let (mut file, _) = setup(case.dataset.as_slice(), &path, case.payload_limits);
                for i in 0..2 {
                    for data in &case.dataset {
                        let js_bytes = serde_json::to_vec(data).unwrap();
                        let entry_bytes = file.read_entry().unwrap().unwrap();
                        assert_eq!(entry_bytes, js_bytes);

                        let d: CaseData = serde_json::from_slice(entry_bytes.as_slice()).unwrap();
                        assert_eq!(&d, data);
                    }
                    assert!(file.read_entry().unwrap().is_none());

                    if i == 0 {
                        // open then read
                        file = BytesIO::open(&path, false).unwrap();
                    }
                }
            }
        }

        #[test]
        fn test_seek() {
            init();
            let case_dir = make_clean_case_dir(module_path!(), "test_seek");

            struct Case {
                path: String,
                payload_limits: u128,
                dataset: Vec<CaseData>,
            }
            let cases = &[Case {
                path: "normal.bytesio".to_owned(),
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
                let (mut file, index) = setup(case.dataset.as_slice(), &path, case.payload_limits);
                for i in 0..2 {
                    for (i, data) in case.dataset.iter().rev().enumerate() {
                        file.seek_entry(&index[&(case.dataset.len() - i - 1)])
                            .unwrap();
                        let js_bytes = serde_json::to_vec(data).unwrap();
                        let entry_bytes = file.read_entry().unwrap().unwrap();
                        assert_eq!(entry_bytes, js_bytes);
                        let d: CaseData = serde_json::from_slice(entry_bytes.as_slice()).unwrap();
                        assert_eq!(&d, data);
                    }
                    assert!(file.read_entry().unwrap().is_some());

                    if i == 0 {
                        // open then seek
                        file = BytesIO::open(&path, false).unwrap();
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
