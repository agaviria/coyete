#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod settings;
pub mod handlers;
pub mod logger;

use settings::Settings;
use std::io;
use std::io::Write;

pub fn initialize() -> rocket::Rocket {
    // load and parse configuration file
    let cfg = Settings::new();

    logger::init_log().unwrap_or_else(|err| {
                                          let _ = writeln!(&mut io::stderr(),
                                                           "Failure at log initialization: {}",
                                                           err);
                                      });
    info!("starting Coyete application v.0.1.0...");

    rocket::ignite()
        .mount("/",
               routes![
               handlers::index,
               ])
        .manage(cfg)
}

// #[cfg(test)]
// mod log_test {
//     use super::logger;

//     #[test]
//     fn log_config() {
//         logger::Logger::init_log(logger::LogStage::Testing);
//     }
// }
