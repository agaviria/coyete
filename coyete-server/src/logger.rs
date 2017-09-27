extern crate log4rs;

pub struct Logger(LogStage);

// LogStage refers to the stage environment used for output and console logging.
pub enum LogStage {
    Testing,
    Development,
    Production,
}

impl Logger {
    pub fn init_log(env: LogStage) {
        match env {
            LogStage::Testing => {
                log4rs::init_file("conf/log/log_test.toml", Default::default())
                    .expect("init test log failure")
            }
            LogStage::Development => {
                log4rs::init_file("conf/log/log_config.toml", Default::default())
                    .expect("init dev log failure")
            }
            LogStage::Production => {
                log4rs::init_file("conf/log/log_config.toml", Default::default())
                    .expect("init production log failure")
            }
        }
    }
}
