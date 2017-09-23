#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate config;
extern crate argon2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate rand;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rwt;
extern crate harsh;
extern crate chrono;
extern crate uuid;
extern crate coyete_data;

pub mod api;
pub mod auth;
pub mod handlers;
pub mod logger;
pub mod settings;
pub mod validation;

use rocket::fairing::AdHoc;
use chrono::Duration;

mod service {
    use config::{Config, File, FileFormat, ConfigError};
    use harsh::{Harsh, HarshBuilder};
    use settings::Token;

    const CFG_DEFAULT: &'static str = "conf/default";

    lazy_static! {
        static ref MASK: Harsh = HarshBuilder::new()
            .length(7)
            .salt(get_nonce().unwrap())
            .init()
            .expect("invalid harsh build");
    }

    pub fn harsh() -> &'static Harsh {
        &MASK
    }

    fn conf_file() -> Config {
        let mut cfg = Config::default();
        cfg.merge(File::from_str(CFG_DEFAULT, FileFormat::Toml))
            .unwrap();

        cfg
    }

    pub fn get_nonce() -> Result<Vec<u8>, ConfigError> {
        let conf = conf_file();
        let search: Token = conf.get("token").unwrap();
        let nonce: Vec<u8> = search.secret.into_bytes();

        Ok(nonce)
    }
}

pub struct RuntimeConfig(Duration);

pub fn initialize() -> rocket::Rocket {
    use settings;
    use coyete_data;

    // initiate development staging log mechanism
    logger::Logger::init_log(logger::LogStage::Development);
    info!("Logger initiated...");

    rocket::ignite()
        .manage(settings::Settings::new())
        .manage(coyete_data::persistance::pg_init_pool_mgr())
        .attach(AdHoc::on_attach(|rocket| {
            let auth_timeout = rocket
                .config()
                .get_int("auth_token_timeout_seconds")
                .unwrap_or("10800");
            let auth_token_duration = Duration::seconds(auth_timeout);
            Ok(rocket.manage(RuntimeConfig(auth_token_duration)))
        }))
    .mount("/",
           routes![
           handlers::index,
           ])

        // TODO:
        //
        // * add catch error handlers: e.g. .catch(errors![]);
        // * mount auth and api routes
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
