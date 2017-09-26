use diesel::pg::PgConnection;
use r2d2::{Config, Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use r2d2::{InitializationError, GetTimeout};

use std::ops::Deref;
use std::fmt;
use std::error::Error as StdError;
use std::convert::From;
use std::sync::Arc;

pub type PgPool = Arc<Pool<ConnectionManager<PgConnection>>>;

/// Initializes pool connection manager for diesel pg.
pub fn init_pg_pool_mgr(db_url_path: &str) -> PgPool {
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(db_url_path);
    Arc::new(Pool::new(config, manager).expect("Postgres pool connection error"))
}

/// Create an adaptor that helps Rocket extract the database connection pool from
/// the request.
impl struct PgConn(PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for PgConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// PgPoolError holds postgres connection pool errors.
/// Diesel errors are implemented in the APIResponse trait found in
/// coyete_server::api::response.
#[derive(Debug)]
pub enum Error {
    Init(InitializationError),
    Conn(GetTimeout),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Init(ref err) => write!(f, "pool initialization error: {}", err),
            Error::Conn(ref err) => write!(f, "pool connection error: {}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Init(ref err) => err.description(),
            Error::Conn(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Init(ref err) => Some(err),
            Error::Conn(ref err) => Some(err),
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
