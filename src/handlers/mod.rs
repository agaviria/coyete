#[get("/")]
fn index() -> &'static str {
    "Hello, Rust + Rocket!"
}
