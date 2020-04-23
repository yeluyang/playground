use std::{
    error,
    fmt::{self, Formatter},
    io, result,
};

extern crate grpcio;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    GRPC(grpcio::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GRPC(err) => err.fmt(f),
            Self::IO(err) => err.fmt(f),
        }
    }
}

impl From<grpcio::Error> for Error {
    fn from(err: grpcio::Error) -> Self {
        Self::GRPC(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}
