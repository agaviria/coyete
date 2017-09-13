use std::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;
use jwt::errors::Error as JwtError;

#[derive(Debug)]
pub enum Error {
    Salt(IoError),
    PassThruValidator(usize),
    Jwt(JwtError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Salt(ref err) => err.fmt(f),
            Error::PassThruValidator(_) => write!(f, "password must not be empty"),
            Error::Mismatch => write!(f, "password does not match"),
            Error::Jwt(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Salt(_) => "cannot generate salt",
            Error::PassThruValidator(_) => "password must not be empty",
            Error::Mismatch => "password does not match",
            Error::Jwt(ref err) => err.description(),
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Salt(err)
    }
}

impl From<JwtError> for Error {
    fn from(err: JwtError) -> Error {
        Error::Jwt(err)
    }
}
