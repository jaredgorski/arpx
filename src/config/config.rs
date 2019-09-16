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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingCfg {
    #[serde(default = "default_false")]
    pub sidebar: bool,
}

pub fn get_px_rc(px_dir: PathBuf) -> Result<Config, Error> {
    let cfg_filename: PathBuf = PathBuf::from("pxrc.yaml");
    let cfg_path: PathBuf = [px_dir, cfg_filename].iter().collect();
    let mut cfg_file: File = File::open(cfg_path).expect("Unable to open config file");

    let mut cfg_file_str = String::new(); 
    cfg_file.read_to_string(&mut cfg_file_str).expect("Unable to read config file");
    let config: Result<Config, Error> = serde_yaml::from_str(&cfg_file_str);

    return config;
}
