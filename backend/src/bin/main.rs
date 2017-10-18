extern crate backend;

fn main() {
    // initialize log and setting configurations.
    // Ignite and lauch Rocket web application.
    backend::initialize().launch();
}
