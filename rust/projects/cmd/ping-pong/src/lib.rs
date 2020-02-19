use std::fmt::{self, Display, Formatter};

mod error;
pub use error::{Error, Result};

#[cfg(test)]
mod tests;

static CRLF: &str = "\r\n";

#[derive(Debug, PartialEq)]
pub enum Protocol {
    SimpleString(String),
    Errors(Error),
    Integers(i128),
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SimpleString(ref s) => write!(f, "+{}{}", s, CRLF),
            Self::Errors(ref err) => write!(f, "-{}{}", err, CRLF),
            Self::Integers(ref i) => write!(f, ":{}{}", i, CRLF),
        }
    }
}

impl Protocol {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

// XXX
// impl<E: std::error::Error> From<E> for Protocol {
//     fn from(err: E) -> Self {
//         Self::Errors(Error::from(err))
//     }
// }

impl From<Vec<u8>> for Protocol {
    fn from(b: Vec<u8>) -> Self {
        let s = String::from_utf8(b).unwrap();
        Self::from(s.as_str())
    }
}

impl<'a> From<&'a str> for Protocol {
    fn from(s: &'a str) -> Self {
        if !s.ends_with(CRLF) {
            return Self::Errors(Error::ParseError("not ends with CRLF(\\r\\n)".to_owned()));
        }
        let s = &s[..s.len() - CRLF.len()];

        let c = s.chars().next().unwrap();
        match c {
            '+' => {
                let content = &s[1..];
                if content.contains(CRLF) {
                    Self::Errors(Error::ParseError(
                        "contains more than one CRLF(\\r\\n) in simple string".to_owned(),
                    ))
                } else {
                    Self::SimpleString(content.to_owned())
                }
            }
            '-' => Self::Errors(Error::from(&s[1..])),
            ':' => s[1..]
                .parse::<i128>()
                .map_or_else(|err| Self::Errors(Error::from(err)), Self::Integers),
            _ => Self::Errors(Error::Unknown(c)),
        }
    }
}
