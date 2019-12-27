extern crate csv;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum CliError {
    IO(io::Error),
    CSV(csv::Error),
    NotFound(String, String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::IO(ref err) => err.fmt(f),
            CliError::CSV(ref err) => err.fmt(f),
            CliError::NotFound(ref path, ref city_name) => {
                writeln!(f, "{} not found in {}", city_name, path)
            }
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CliError::IO(ref err) => Some(err),
            CliError::CSV(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IO(err)
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> CliError {
        CliError::CSV(err)
    }
}
