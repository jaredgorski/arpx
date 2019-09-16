use serde::{Serialize, Deserialize};
use std::path::{PathBuf};

pub mod config;
pub mod profile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub config: config::Config,
    pub profile: profile::Profile,
}

pub fn get_pmux_cfg() -> Cfg {
    let home_dir: PathBuf = match dirs::home_dir() {
        Some(dir) => dir,
        _ => panic!(),
    };
    let pmux_dirname: PathBuf = PathBuf::from(".pmux");
    let pmux_dir: PathBuf = [home_dir, pmux_dirname].iter().collect();

    let config = match config::get_pmux_rc(pmux_dir.clone()) {
        Ok(config) => config,
        Err(error) => panic!(error),
    };

    let profile = match profile::get_pmux_pr(pmux_dir.clone(), PathBuf::from(config.profile.clone())) {
        Ok(profile) => profile,
        Err(error) => panic!(error),
    };

    let cfg: Cfg = Cfg {
        config: config,
        profile: profile,
    };

    return cfg;
}

pub fn default_cwd() -> String {
    ".".to_string()
}

pub fn default_empty_string() -> String {
    "".to_string()
}

pub fn default_false() -> bool {
    false
}

pub fn default_empty_vec_string() -> Vec<String> {
    Vec::new()
}
