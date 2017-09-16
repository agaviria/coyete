use std::fmt;
use std::error::Error as StdError

use r2d2::{InitializationError, GetTimeout};

#[derive(Debug)]
pub enum Error {
    Init(InitializationError),
    Conn(GetTimeout),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Init::(ref err) => err.fmt(f),
            Error::Conn::(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Init::(ref err) => err.description(),
            Error::Conn::(ref err) => err.description(),
        }
    }
}

impl From<InitializationError> for Error {
    fn from(err: InitializationError) -> Error {
        Error::Init(err)
    }
}

impl From<GetTimeout> for Error {
    fn from(err: GetTimeout) -> Error {
        Error::Conn(err)
    }
}
