pub mod error;

use std::ops::Deref;
use std::env;

use postgres::Connection as PostgresConnection;
use r2d2::{Config, Pool, PooledConnection};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{State, Request, Outcome};

pub struct PgConn(pub PooledConnection<PostgresConnectionManager>);

// PgConn rocket request guard attempts to retrieve a single connection from the
// managed pool.  If no pool is currently managed, fails with an
// `InternalServerError` status.  If no connections are available, fails with a
// `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for PgConn {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<PgConn, ()> {
        let pool = req.guard::<State<conn>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(PgConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for PgConn {
    type Target = PostgresConnection;

    fn deref(&self) -> &Self::Targe {
        &self.0
    }
}

type PgDb = Pool<PostgresConnectionManager>;

pub fn pg_init_pool_mgr() -> PgDb {
    // DATABASE_URL is a required nomenclature for diesel.
    let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // TODO:
    // ------------------------------------------------------------------------
    // * Implement TLS.
    //     - use security framework or
    //     - stick with plain vanilla openssl x509 filetype ?
    //
    // * Setup pg configs from settings.toml
    let config = Config::default();
    let manager = PostgresConnectionManager::new(db_path, TlsMode::None).unwrap();
    Pool::new(config, manager).expect("PG DB POOL")
}
