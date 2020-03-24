use std::{
    ffi::OsString,
    fmt::{self, Display, Formatter},
    io,
    path::Path,
};

extern crate segment_io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    HeaderMissing(String),
    EmptyFile(String),
    InvalidPath(OsString),
    IncompleteWrite(String),
    SegmentsFile(segment_io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(err) => err.fmt(f),
            Self::HeaderMissing(path) => write!(
                f,
                "failed to open log structed file: miss header, path={}",
                path,
            ),
            Self::EmptyFile(path) => write!(
                f,
                "failed to open log structed file: empty file even header, path={}",
                path,
            ),
            Self::InvalidPath(path) => write!(
                f,
                "failed to open log structed file: invalid path, path={:?}",
                path,
            ),
            Self::IncompleteWrite(path) => write!(
                f,
                "failed to open log structed file: incomplete write, path={}",
                path,
            ),
            Self::SegmentsFile(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<segment_io::Error> for Error {
    fn from(err: segment_io::Error) -> Self {
        Error::SegmentsFile(err)
    }
}

pub fn path_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    path.as_ref()
        .to_str()
        .map(|s| s.to_owned())
        .ok_or_else(|| Error::InvalidPath(path.as_ref().as_os_str().to_os_string()))
}
