use std::fmt;
// use std::num::ParseIntError;
use std::error::Error as StdError;
use std::result::Result as StdResult;

use argon2::Error as HashErrorKind;
use r2d2::{InitializationError, GetTimeout};
use diesel::result::{Error as DieselErrorKind, ConnectionError};
// use jwt::Error as TokenError;

pub mod kind;

use self::kind::InputErrorKind;

pub type Result<T> = StdResult<T, Error>;

pub enum Error {
    /// An error while hashing a string.
    HashError(HashErrorKind),
    /// An error occuring from Diesel.
    DieselError(DieselErrorKind),
    /// An error on authentication format.
    InputError(InputErrorKind),
    /// An error for web tokens.
    // JwtError(TokenError),
    /// A diesel connection error.
    PgConnError(ConnectionError),
    /// An error while attempting to initialize a database connection pool.
    PgInitError(InitializationError),
    /// Timeout while attempting to retrieve a database connection from pool.
    PgTimeoutError(GetTimeout),
    ParseError,
    PasswordMatchError,
    PermissionError,
    UserNotVerifiedError,
}

impl Error {
    fn input_description(input_error: &InputErrorKind) -> &str {
        match *input_error {
            InputErrorKind::Password(_) => "Invalid password",
            InputErrorKind::Email(_) => "Invalid email",
            InputErrorKind::Authenticator => "Invalid authentication format",
            InputErrorKind::PermissionName => "Invalid permission name",
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DieselError(ref db_error) => db_error.description(),
            Error::InputError(ref input_error) => Error::input_description(input_error),
            Error::HashError(ref hash_error) => hash_error.description(),
            // Error::JwtError(ref jwt_error) => jwt_error.description(),
            Error::ParseError => "Could not parse data from string",
            Error::PasswordMatchError => "Passwords do not match",
            Error::PermissionError => "Not allowed to perform this action",
            Error::UserNotVerifiedError => "User is not verified",
            Error::PgConnError(ref diesel_conn_error) => diesel_conn_error.description(),
            Error::PgInitError(ref r2d2_init_error) => r2d2_init_error.description(),
            Error::PgTimeoutError(ref r2d2_timeout_error) => r2d2_timeout_error.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::DieselError(ref db_error) => Some(db_error),
            Error::HashError(ref hash_error) => Some(hash_error),
            // Error::JwtError(ref jwt_error) => Some(jwt_error),
            Error::PgConnError(ref r2d2_timeout_error) => Some(r2d2_timeout_error),
            Error::PgInitError(ref r2d2_init_error) => Some(r2d2_init_error),
            Error::PgTimeoutError(ref r2d2_timeout_error) => Some(r2d2_timeout_error),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DieselError(ref err) => write!(f, "Database error: {}", err),
            Error::HashError(ref err) => write!(f, "Hash method error: {}", err),
            Error::InputError(ref err) => write!(f, "Auth format error: {}", err.to_string()),
            // Error::JwtError(ref err) => write!(f, "Web token error: {}", err),
            Error::PgConnError(ref err) => write!(f, "Pool connection  err: {}", err),
            Error::PgInitError(ref err) => write!(f, "Pool initialization err: {}", err),
            Error::PgTimeoutError(ref err) => {
                write!(f, "Timed out connecting to database: {}", err)
            }
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<DieselErrorKind> for Error {
    fn from(e: DieselErrorKind) -> Error {
        Error::DieselError(e)
    }
}

impl From<InitializationError> for Error {
    fn from(e: InitializationError) -> Error {
        Error::PgInitError(e)
    }
}

impl From<ConnectionError> for Error {
    fn from(e: ConnectionError) -> Error {
        Error::PgConnError(e)
    }
}

impl From<GetTimeout> for Error {
    fn from(e: GetTimeout) -> Error {
        Error::PgTimeoutError(e)
    }
}
impl From<HashErrorKind> for Error {
    fn from(e: HashErrorKind) -> Error {
        Error::HashError(e)
    }
}

// impl From<TokenError> for Error {
//     fn from(e: TokenError) -> Error {
//         Error::JwtError(e)
//     }
// }

// impl From<ParseIntError> for Error {
//     fn from(_: ParseIntError) -> Error {
//         Error::ParseError
//     }
// }
