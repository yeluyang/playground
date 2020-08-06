use std::{
    error,
    fmt::{self, Display, Formatter},
    result,
};

use crate::EndPoint;

extern crate grpcio;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RPC(EndPoint, RPCError), // TODO add endpoint into error
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::RPC(endpoint, err) => write!(f, "RPC error from {}: {}", endpoint, err),
        }
    }
}

impl From<(EndPoint, RPCError)> for Error {
    fn from(err: (EndPoint, RPCError)) -> Self {
        Self::RPC(err.0, err.1)
    }
}

#[derive(Debug)]
pub enum RPCError {
    Unknown(String),
    GRPC(grpcio::Error),
}

impl error::Error for RPCError {}

impl Display for RPCError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(err) => write!(f, "unknown RPC: {}", err),
            Self::GRPC(err) => write!(f, "gRPC: {}", err),
        }
    }
}

impl From<String> for RPCError {
    fn from(err: String) -> Self {
        Self::Unknown(err)
    }
}

impl From<grpcio::Error> for RPCError {
    fn from(err: grpcio::Error) -> Self {
        Self::GRPC(err)
    }
}
