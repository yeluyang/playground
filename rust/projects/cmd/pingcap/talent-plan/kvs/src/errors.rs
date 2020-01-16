use std::{
    error,
    fmt::{self, Display, Formatter},
    io,
};

extern crate lsmt;

/// Error
#[derive(Debug)]
pub enum Error {
    /// unknown error occur
    Unknown,
    /// key not found in data-storage
    KeyNotFound(String),
    /// error from LogStructuredMergeTree
    LSMTError(lsmt::Error),
    /// IO error
    IO(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Unknown => write!(f, "unknown or impossible error occurred"),
            Error::KeyNotFound(ref key) => write!(f, "`key = {}` not found in storage", key),
            Error::LSMTError(ref err) => err.fmt(f),
            Error::IO(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<lsmt::Error> for Error {
    fn from(err: lsmt::Error) -> Self {
        Error::LSMTError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
