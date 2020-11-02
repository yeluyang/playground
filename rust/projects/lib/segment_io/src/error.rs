use std::{
    error,
    fmt::{self, Formatter},
    io,
    path::PathBuf,
    result,
};

use crate::Version;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MetaMissing(PathBuf),
    Incompatible(Version, Version),
    FileExisted(PathBuf),
    EntryMismatch(u128, u128),
    MeetIncompleteEntry(u128, u128),
    WriteOnReadOnlyFile(PathBuf),
    PayloadLimitZero,
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaMissing(path) => write!(f, "header of file missing: {:?}", path),
            Self::Incompatible(current_version, file_version) => write!(
                f,
                "incompatible version, major version should be equal: current={}, got={}",
                current_version, file_version
            ),
            Self::EntryMismatch(expect, actual) => write!(
                f,
                "sequence of entry mismatch: expect={}, actual={}",
                expect, actual
            ),
            Self::FileExisted(path) => write!(f, "file already existed: {:?}", path),
            Self::MeetIncompleteEntry(expect, actual) => write!(
                f,
                "meet incomplete entry: expect {} frames, got {}",
                expect, actual
            ),
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
