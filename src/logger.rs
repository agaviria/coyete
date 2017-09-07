extern crate log4rs;

// LogStage refers to the ecosystem used for output and console logging.
// Development staging || Production staging.
pub enum LogStage {
    Development,
    Production,
}

pub struct Logger {}

impl Logger {
    pub fn init_logger(env: LogStage) {
        match env {
            LogStage::Development => log4rs::init_file("log/log_config.toml", Default::default()).expect("Development stage log mechanism failed to initialize"),
            LogStage::Production => log4rs::init_file("log/log_config.toml", Default::default()).expect("Production stage log mechanism failed to initialize"),
        }
    }
}
