#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate accord;
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

pub fn initialize() -> rocket::Rocket {
    // load and parse configuration file
    let cfg = Settings::new();

    // initiate development staging log mechanism
    logger::Logger::init_log(logger::LogStage::Development);
    info!("Logger initiated...");

    rocket::ignite()
        .mount("/",
               routes![
               handlers::index,
               ])
        .manage(cfg)
}

#[cfg(test)]
mod log_test {
    use super::initialize;
    use std::env::current_dir;
    use super::logger;

    #[test]
    fn log_config() {
        logger::Logger::init_log(logger::LogStage::Testing);
        error!("running in {:?}", current_dir().unwrap().display());
        error!("fatal");
        warn!("warn");
        info!("info");
        debug!("debug");
        trace!("trace");
    }
}
