#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate coyete_server;


fn main() {
    // initialize log and setting configurations.
    // Ignite and lauch Rocket web application.
    coyete::initialize().launch();
}
