use std::env;
use config::{ConfigError, Config, File, Environment};

const CFG_DEFAULT: &'static str = "conf/default";

#[derive(Debug, Deserialize)]
pub struct Application {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    application: Application,
}

// Settings uses a hierarchical configuration based on staging environment
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();

        // start with merging the 'default' configuration file
        cfg.merge(File::with_name(CFG_DEFAULT))?;

        // Looks for environment RUN_MODE == production
        // Otherwise, defaults to 'development' env if not set
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        cfg.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Adds settings from the env file (with a prefix of APP)
        // e.g. `APP_DEBUG=1 ./target/app` would set the `debug` key
        cfg.merge(Environment::with_prefix("app"))?;

        // deserialize and freeze's the entire configuration
        cfg.try_into()
    }
}
