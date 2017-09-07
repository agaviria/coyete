use std::io::prelude::*;
use std::fs::File;
use std::env;

use toml;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
}

pub fn parse() -> Config {
    let cfg_filepath =
        env::var("CONFIG_FILE_PATH").expect("CONFIG_FILE_PATH env variable must be set.");

    let mut f = File::open(cfg_filepath).expect("Cannot open configuration file.");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Cannot read configuration file contents.");

    let cfg: Config = toml::from_str(content.as_str()).expect("Cannot parse configuration file.");

    return cfg;
}
