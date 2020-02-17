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
    Arrays(Vec<Protocol>),
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SimpleString(ref s) => write!(f, "+{}{}", s, CRLF),
            Self::Errors(ref err) => write!(f, "-{}{}", err, CRLF),
            Self::Integers(ref i) => write!(f, ":{}{}", i, CRLF),
            Self::Arrays(ref array) => {
                let number = format!("{}{}", array.len(), CRLF);
                let mut content = String::new();
                for a in array {
                    content.push_str(a.to_string().as_str());
                }
                write!(f, "*{}{}{}", number, content, CRLF)
            }
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

        match s.chars().next().unwrap() {
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
            '*' => {
                let mut content = &s[1..];
                if let Some(i) = content.find(CRLF) {
                    match content[..i].parse::<i128>() {
                        Ok(num) => {
                            content = &content[i + CRLF.len()..];
                            let mut array: Vec<Protocol> = Vec::new();
                            while let Some(i) = content.find(CRLF) {
                                // FIXME: bug occur when array nest in array
                                array.push(Self::from(&content[..i + CRLF.len()]));
                                content = &content[i + CRLF.len()..];
                            }
                            if array.len() != num as usize {
                                Self::Errors(Error::MissElement(format!(
                                    "expect {}, but get {}",
                                    num,
                                    array.len()
                                )))
                            } else {
                                Self::Arrays(array)
                            }
                        }
                        Err(err) => Self::Errors(Error::from(err)),
                    }
                } else {
                    Self::Errors(Error::ParseError("missing numeric of element".to_owned()))
                }
            }
            _ => unimplemented!(),
        }
    }
}
