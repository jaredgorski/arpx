use serde::{Serialize, Deserialize};
use std::path::{PathBuf};

pub mod config;
pub mod profile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub config: config::Config,
    pub profile: profile::Profile,
}

pub fn get_px_cfg() -> Cfg {
    let home_dir: PathBuf = match dirs::home_dir() {
        Some(dir) => dir,
        _ => panic!(),
    };
    let px_dirname: PathBuf = PathBuf::from(".px");
    let px_dir: PathBuf = [home_dir, px_dirname].iter().collect();

    let config = match config::get_px_rc(px_dir.clone()) {
        Ok(config) => config,
        Err(error) => panic!(error),
    };

    let profile = match profile::get_px_pr(px_dir.clone(), PathBuf::from(config.profile.clone())) {
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
