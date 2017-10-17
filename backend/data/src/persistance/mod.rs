use std::ops::Deref;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Generate a new r2d2 Pool for connection management in rocket sessions.
/// This object will be handed to the rocket `.manage()` managed state handler.
pub fn init_db_pool() -> Pool {
    let config = r2d2::Config::default();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}


/// A helper struct to pass the `PooledConnection` from r2d2 to the request function.
/// It implements the `Deref` trait to easily get the `PgConnection` by using `*`
pub struct PgConn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for PgConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PgConn {
    pub fn new(pool: r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> PgConn {
        PgConn(pool)
    }
}
