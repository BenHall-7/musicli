use std::error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum Error {
    OutOfBounds(&'static str),
    IO(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::OutOfBounds(msg) => write!(f, "OutOfBounds error: {:?}", msg),
            Error::IO(e) => write!(f, "IO error: {:?}", e),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::OutOfBounds(_) => None,
            Error::IO(e) => Some(e),
        }
    }
}
