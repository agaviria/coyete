use std::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;

use self::ErrorKind::*;
use super::persistance::Error as StorageError;
use chrono::{DateTime, Utc};
use rwt::RwtError;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    BadToken(RwtError),
    Expired(DateTime<Utc>),
    MinLen(usize),
    Mismatch,
    Salt(IoError),
    Persistance(StorageError),
    Unauthorized,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    cause: Option<Box<StdError>>,
    description: &'static str,
}

impl Error {
    pub fn new(kind: ErrorKind, cause: Box<StdError>, description: &str) -> Error {
        Error {
            kind: kind,
            cause: Some(cause),
            description: description,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            BadToken => write!(f, "bad session token"),
            Expired(ref expiration) => write!(f, "expired: {}", expiration),
            MinLen => write!(f, "password does not meet minimum length requirement"),
            Mismatch => write!(f, "password verification failure"),
            Salt => write!(f, "cannot generate salt"),
            Persistance => write!(f, "persistance storage"),
            Unauthorized => write!(f, "unauthorized request"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.description
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error {
            kind: Salt,
            cause: Some(Box::new(err)),
            description: "Salt generator failure",
        }
    }
}

impl From<RwtError> for Error {
    fn from(err: RwtError) -> Self {
        Error {
            kind: BadToken,
            cause: Some(Box::new(err)),
            description: "Bad token",
        }
    }
}

impl From<StorageError> for Error {
    fn from(err: StorageError) -> Self {
        Error {
            kind: Persistance,
            cause: Some(Box::new(err)),
            description: err.description().to_string(),
        }
    }
}
