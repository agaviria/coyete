extern crate rocket;
extern crate diesel;
extern crate backend;
extern crate coyete_data;
extern crate dotenv;
extern crate chrono;

use rocket::local::Client;
use diesel::pg::PgConnection;
use diesel::connection::Connection;

fn new_client() -> Client {
    let r = backend::initialize();
    Client::new(r).expect("valid rocket instance")
}

fn get_test_db_conn() -> PgConnection {
    use std::env;
    use dotenv::dotenv;

    dotenv().ok();

    let url =
        env::var("DATABASE_URL").expect("DATABASE_URL env must be set in order to run tests.");
    let conn = PgConnection::establish(&url).unwrap();
    conn.begin_test_transaction().unwrap();

    conn
}

#[cfg(test)]
mod backend_test {
    use rocket::http::{Status, ContentType};
    use super::{new_client, get_test_db_conn};
    use backend::handlers;

    #[test]
    fn it_shows_index() {
        let client = new_client();
        let mut resp = client.get("/").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type().expect("valid Content-Type"),
                   ContentType::Plain);
        let body = resp.body_string();
        assert_eq!(body.expect("body content"), handlers::index());
    }

    #[test]
    fn read_from_pg() {
        use diesel::prelude::*;

        let conn = get_test_db_conn();
        let users_count = conn.execute("SELECT * FROM users").unwrap();
        assert_eq!(0, users_count);
    }
}
