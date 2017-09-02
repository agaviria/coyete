#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust + Rocket!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    // initializes rocket's routes
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let client = Client::new(rocket()).unwrap();
        let mut r = client.get("/").dispatch();
        assert_eq!(r.status(), Status::Ok);
        assert_eq!(r.body_string(), Some("Hello, Rust + Rocket!".into()));
    }
}
