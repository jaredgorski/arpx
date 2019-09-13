use serde::{Serialize, Deserialize};
use std::path::{PathBuf};

pub mod config;
pub mod profile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub config: config::Config,
    pub profile: profile::Profile,
}

pub fn get_tom_cfg() -> Cfg {
    let home_dir: PathBuf = match dirs::home_dir() {
        Some(dir) => dir,
        _ => panic!(),
    };
    let tom_dirname: PathBuf = PathBuf::from(".tom");
    let tom_dir: PathBuf = [home_dir, tom_dirname].iter().collect();

    let config = match config::get_tom_rc(tom_dir.clone()) {
        Ok(config) => config,
        Err(error) => panic!(error),
    };

    let profile = match profile::get_tom_pr(tom_dir.clone(), PathBuf::from(config.profile.clone())) {
        Ok(profile) => profile,
        Err(error) => panic!(error),
    };

    let cfg: Cfg = Cfg {
        config: config,
        profile: profile,
    };

    return cfg;
}
