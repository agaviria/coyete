#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod config;
pub mod handlers;

pub fn initialize() -> rocket::Rocket {
    // configuration is under development therefore we assign unused variable.
    // consider implementing struct error messaging.
    let cfg = config::parse();

    rocket::ignite()
        .mount("/",
               routes![
               handlers::index,
               ])
        .manage(cfg)
}

#[cfg(test)]
mod test {
    use super::initialize;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index_handler() {
        let client = Client::new(initialize()).unwrap();
        let mut resp = client.get("/").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.body_string(), Some("Hello, Rust + Rocket!".into()));
    }
}
