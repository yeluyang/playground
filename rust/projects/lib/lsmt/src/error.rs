use std::{
    ffi::OsString,
    fmt::{self, Display, Formatter},
    io,
    path::Path,
};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    HeaderMissing(String),
    EmptyFile(String),
    InvalidPath(OsString),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IO(ref err) => err.fmt(f),
            Self::HeaderMissing(ref path) => write!(
                f,
                "failed to open log structed file: miss header, path={}",
                path,
            ),
            Self::EmptyFile(ref path) => write!(
                f,
                "failed to open log structed file: empty file, path={}",
                path,
            ),
            Self::InvalidPath(ref path) => write!(
                f,
                "failed to open log structed file: invalid path, path={:?}",
                path,
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn get_path_string<P: AsRef<Path>>(path: P) -> Result<String> {
    path.as_ref()
        .to_str()
        .map(|s| s.to_owned())
        .ok_or(Error::InvalidPath(path.as_ref().as_os_str().to_os_string()))
}
