use std::{
    fmt::{self, Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IO(ref err) => err.fmt(f),
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
