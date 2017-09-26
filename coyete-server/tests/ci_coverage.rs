extern crate rocket;
extern crate coyete;

use rocket::local::Client;
use rocket::http::{Status, ContentType};

fn new_client() -> Client {
    let r = coyete::initialize();
    Client::new(r).expect("valid rocket instance")
}

#[cfg(test)]
mod backend_test {
    use rocket::http::{Status, ContentType};
    use std::env::current_dir;
    // use super::logger;
    use super::new_client;
    use coyete;

    #[test]
    fn it_shows_index() {
        let client = new_client();
        let mut resp = client.get("/").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type().expect("valid Content-Type"),
                   ContentType::Plain);
        let body = resp.body_string();
        assert_eq!(body.expect("body content"), coyete::handlers::index());
    }

}
