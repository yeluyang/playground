use std::{
    error,
    ffi::OsString,
    fmt::{self, Display, Formatter},
    io, string,
};

extern crate serde_json;

extern crate sled;

extern crate lsmt;

/// Error
#[derive(Debug)]
pub enum Error {
    /// unknown error occur
    Simple(String),
    /// key not found in data-storage
    KeyNotFound(String),
    /// data not found
    DataNotFound(String),
    /// error from LogStructuredMergeTree
    LSMTError(lsmt::Error),
    /// IO error
    IO(io::Error),
    /// TODO
    InvalidPath(OsString),
    /// TODO
    SerdeJSON(serde_json::Error),
    /// TODO
    SledError(sled::Error),
    /// TODO
    ParseUTF8Error(string::FromUtf8Error),
    /// TODO
    EngineMismatch { exist: String, got: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Simple(s) => write!(f, "{}", s),
            Error::KeyNotFound(key) => write!(f, "`key = {}` not found in storage", key),
            Error::DataNotFound(key) => write!(f, "data of key={} not found in storage", key),
            Error::InvalidPath(path) => write!(f, "path={:?} invalid", path),
            Error::LSMTError(err) => err.fmt(f),
            Error::IO(err) => err.fmt(f),
            Error::SerdeJSON(err) => err.fmt(f),
            Self::SledError(err) => err.fmt(f),
            Self::ParseUTF8Error(err) => err.fmt(f),
            Self::EngineMismatch { exist, got } => {
                write!(f, "engine={} already exist, but got {}", exist, got)
            }
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJSON(err)
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Self::SledError(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Self::ParseUTF8Error(err)
    }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
