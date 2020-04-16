use std::{
    error,
    fmt::{self, Formatter},
    result,
};

extern crate grpcio;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GRPC(grpcio::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GRPC(err) => err.fmt(f),
        }
    }
}

impl From<grpcio::Error> for Error {
    fn from(err: grpcio::Error) -> Self {
        Self::GRPC(err)
    }
}
