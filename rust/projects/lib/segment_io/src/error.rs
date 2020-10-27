use std::{
    error,
    fmt::{self, Formatter},
    io,
    path::PathBuf,
    result,
};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MetaMissing(PathBuf),
    FileExisted(PathBuf),
    ReadFromMiddle(u128, u128),
    WriteOnReadOnlyFile(PathBuf),
    PayloadLimitZero,
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaMissing(path) => write!(f, "header of file missing: {:?}", path),
            Self::FileExisted(path) => write!(f, "file already existed: {:?}", path),
            Self::ReadFromMiddle(seq, total) => {
                write!(f, "read from middle segment: {}/{}", seq, total)
            }
            Self::WriteOnReadOnlyFile(path) => {
                write!(f, "write segments on read-only file: {:?}", path)
            }
            Self::PayloadLimitZero => write!(f, "limit of bytes of payload must greater than zero"),
            Self::IO(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}
