use std::{
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Internal(String),
    ParseError(String),
    MissElement(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(ref s) => write!(f, "INTERNAL {}", s),
            Self::ParseError(ref s) => write!(f, "PARSEERROR {}", s),
            Self::MissElement(ref s) => write!(f, "MISSELEMENT {}", s),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        if let Some(i) = s.find(' ') {
            match &s[..i] {
                "INTERNAL" => Error::Internal(s[i + 1..].to_owned()),
                "PARSEERROR" => Error::ParseError(s[i + 1..].to_owned()),
                "MISSELEMENT" => Error::MissElement(s[i + 1..].to_owned()),
                _ => unimplemented!(),
            }
        } else {
            Error::Internal(
                "missing seperate ' ' between 'Error Type' and 'Error Message'".to_owned(),
            )
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseError(err.to_string())
    }
}

// XXX
// impl<E: std::error::Error> From<E> for Error {
//     fn from(err: E) -> Self {
//         Error::Unknown(err.to_string())
//     }
// }
