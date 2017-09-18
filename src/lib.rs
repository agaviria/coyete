#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate argon2;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate config;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate postgres;
extern crate toml;
extern crate rand;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rwt;
extern crate harsh;
extern crate chrono;

pub mod auth;
pub mod handlers;
pub mod logger;
pub mod persistance;
pub mod settings;

mod service {
    // use std::sync::RwLock;

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

        // static ref CFG: RwLock<Config> = RwLock::new(conf_file());
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

pub fn initialize() -> rocket::Rocket {
    use settings;
    use persistance;

    // initiate development staging log mechanism
    logger::Logger::init_log(logger::LogStage::Development);
    info!("Logger initiated...");

    rocket::ignite()
        .mount("/",
               routes![
               handlers::index,
               ])
        .manage(settings::Settings::new())
        .manage(persistance::pg_init_pool_mgr())
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
