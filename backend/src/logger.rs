extern crate log4rs;

use std::error::Error;
use log::LogLevelFilter;
use log4rs::config::{Appender, Config, Root};
use log4rs::init_config;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;

pub fn init_log() -> Result<(), Box<Error>> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {h([{l}])}: {m}{n}")))
        .build();

    let logfile = try!(FileAppender::builder()
                 .encoder(Box::new(PatternEncoder::new("{d(%m-%d-%Y %H:%M:%S)} [{l}]: {m}{n}")))
                 .append(true)
                 .build("logs/app_backend.log"));

    let config = try!(Config::builder()
                          .appender(Appender::builder().build("stdout", Box::new(stdout)))
                          .appender(Appender::builder().build("logfile", Box::new(logfile)))
                          .build(Root::builder()
                                     .appender("stdout")
                                     .appender("logfile")
                                     .build(LogLevelFilter::Debug)));

    init_config(config)?;
    Ok(())

}
