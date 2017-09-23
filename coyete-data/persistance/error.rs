use std::fmt;
use std::error::Error as StdError;
use std::convert::From;

use r2d2::{InitializationError, GetTimeout};

/// PgPoolError holds postgres connection pool errors.
/// Diesel errors are implemented in the APIResponse trait found in
/// coyete::api::response, and return an internal server error.
#[derive(Debug)]
pub enum PgPoolError {
    Init(InitializationError),
    Conn(GetTimeout),
}

impl fmt::Display for PgPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PgPoolError::Init(ref err) => write!(f, "pool initialization error: {}", err),
            PgPoolError::Conn(ref err) => write!(f, "pool connection error: {}", err),
        }
    }
}

impl StdError for PgPoolError {
    fn description(&self) -> &str {
        match *self {
            PgPoolError::Init(ref err) => err.description(),
            PgPoolError::Conn(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            PgPoolError::Init(ref err) => Some(err),
            PgPoolError::Conn(ref err) => Some(err),
        }
    }
}

impl From<InitializationError> for PgPoolError {
    fn from(err: InitializationError) -> PgPoolError {
        PgPoolError::Init(err)
    }
}

impl From<GetTimeout> for PgPoolError {
    fn from(err: GetTimeout) -> PgPoolError {
        PgPoolError::Conn(err)
    }
}
