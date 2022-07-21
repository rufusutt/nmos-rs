use std::error::Error as StdError;
use std::fmt::{Error as FmtError, self};
use std::io::Error as IoError;
use std::result::Result as StdResult;

use hyper::Error as HyperError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Format(FmtError),
    Io(IoError),
    Hyper(HyperError),
}

impl From<FmtError> for Error {
    fn from(e: FmtError) -> Self {
        Error::Format(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Self {
        Error::Hyper(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Format(e) => fmt::Display::fmt(&e, f),
            Error::Io(e) => fmt::Display::fmt(&e, f),
            Error::Hyper(e) => fmt::Display::fmt(&e, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Format(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Hyper(e) => Some(e),
        }
    }
}
