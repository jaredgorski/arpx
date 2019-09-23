use serde::{Serialize, Deserialize};
use serde_yaml::{Error};
use std::fs::{File};
use std::path::{PathBuf};
use std::io::prelude::*;
use crate::config::{
    default_empty_string,
    default_false,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_empty_string")]
    pub profile: String,
    pub logging: LoggingCfg,
}

impl Config {
    pub fn new() -> Config {
        Config {
            profile: "".to_string(),
            logging: LoggingCfg::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingCfg {
    #[serde(default = "default_false")]
    pub sidebar: bool,
}

impl LoggingCfg {
    pub fn new() -> LoggingCfg {
        LoggingCfg {
            sidebar: false,
        }
    }
}

pub fn get_sym_rc(sym_dir: PathBuf) -> Result<Config, Error> {
    let cfg_filename: PathBuf = PathBuf::from("sym.conf.yaml");
    let cfg_path: PathBuf = [sym_dir, cfg_filename].iter().collect();
    let mut cfg_file: File = File::open(cfg_path).expect("Unable to open config file");

    let mut cfg_file_str = String::new(); 
    cfg_file.read_to_string(&mut cfg_file_str).expect("Unable to read config file");
    let config: Result<Config, Error> = serde_yaml::from_str(&cfg_file_str);

    return config;
}
