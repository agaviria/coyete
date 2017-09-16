pub mod error;

use r2d2::{Config, ManageConnection, Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use self::error::Error;

pub struct Persistance<T: ManageConnection> {
    pool: Pool<T>,
}

impl<T: ManageConnection> Persistance<T> {
    pub fn new(manager: T) -> Result<Persistance<T>, Error> {
        let config = Config::default();
        let pool = Pool::new(config, manager)?;

        Ok(Persistance { pool: pool })
    }

    pub fn get_conn(&self) -> Result<PooledConnection<T>, Error> {
        let conn = self.pool.get()?;

        Ok(conn)
    }
}

pub type PgDatabase = Persistance<PostgresConnectionManager>;
