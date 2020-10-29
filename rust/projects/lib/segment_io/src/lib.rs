use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Display, Formatter},
    io::Cursor,
    mem, result,
};

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
type Endian = LittleEndian;

#[macro_use]
extern crate log;

mod error;
mod seg_file;
mod segment;

#[cfg(test)]
mod tests;

// public lists
pub use error::{Error, Result};
pub use seg_file::BytesIO;

// protocol version of BytesIO file
const VERSION_MAJOR: u128 = 1;
const VERSION_MINOR: u128 = 0;
const VERSION_PATCH: u128 = 0;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Version {
    major: u128,
    minor: u128,
    patch: u128,
}
const VERSION_BYTES: usize = mem::size_of::<Version>();
const CURRENT_VERSION: Version = Version::new();

impl Version {
    const fn new() -> Version {
        return Version {
            major: VERSION_MAJOR,
            minor: VERSION_MINOR,
            patch: VERSION_PATCH,
        };
    }

    fn is_compatible(&self) -> bool {
        self.major == VERSION_MAJOR
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.write_u128::<Endian>(self.major)?;
        bytes.write_u128::<Endian>(self.minor)?;
        bytes.write_u128::<Endian>(self.patch)?;

        assert_eq!(bytes.len(), VERSION_BYTES);

        Ok(bytes)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert_eq!(bytes.len(), VERSION_BYTES);

        let mut rdr = Cursor::new(Vec::from(bytes));
        let major = rdr.read_u128::<Endian>()?;
        let minor = rdr.read_u128::<Endian>()?;
        let patch = rdr.read_u128::<Endian>()?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl TryFrom<Vec<u8>> for Version {
    type Error = Error;
    fn try_from(bytes: Vec<u8>) -> result::Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryInto<Vec<u8>> for Version {
    type Error = Error;
    fn try_into(self) -> result::Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}
