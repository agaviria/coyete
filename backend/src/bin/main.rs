extern crate coyete_server;

fn main() {
    // initialize log and setting configurations.
    // Ignite and lauch Rocket web application.
    coyete_server::initialize().launch();
}
