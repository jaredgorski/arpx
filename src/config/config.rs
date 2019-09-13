use serde::{Serialize, Deserialize};
use serde_yaml::{Error};
use std::fs::{File};
use std::path::{PathBuf};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub profile: String,
    pub logging: LoggingCfg,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingCfg {
    pub sidebar: bool,
}

pub fn get_tom_rc(tom_dir: PathBuf) -> Result<Config, Error> {
    let cfg_filename: PathBuf = PathBuf::from("tomrc.yaml");
    let cfg_path: PathBuf = [tom_dir, cfg_filename].iter().collect();
    let mut cfg_file: File = File::open(cfg_path).expect("Unable to open config file");

    let mut cfg_file_str = String::new(); 
    cfg_file.read_to_string(&mut cfg_file_str).expect("Unable to read config file");
    let config: Result<Config, Error> = serde_yaml::from_str(&cfg_file_str);

    return config;
}
