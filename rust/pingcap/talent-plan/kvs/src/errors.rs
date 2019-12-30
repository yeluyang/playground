use std::{
    error,
    fmt::{self, Display, Formatter},
};

/// Error
#[derive(Debug)]
pub enum Error {
    /// unknown error occur
    Unknown,
    /// key not found in data-storage
    KeyNotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Unknown => write!(f, "unknown or impossible error occurred"),
            Error::KeyNotFound(ref key) => write!(f, "`key = {}` not found in storage", key),
        }
    }
}

impl error::Error for Error {}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
