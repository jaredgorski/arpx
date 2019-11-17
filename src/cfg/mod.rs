use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod profile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub profile: profile::Profile,
}

impl Default for Cfg {
    fn default() -> Self {
        Self::new()
    }
}

impl Cfg {
    pub fn new() -> Cfg {
        let profile = profile::Profile::new();

        Cfg { profile }
    }
}

pub fn get_tog_cfg(profile_path: &str) -> Cfg {
    let prof_dir: PathBuf = PathBuf::from(".");
    let prof_path: String = if profile_path == "default" {
        "".to_string()
    } else {
        profile_path.to_string()
    };

    let profile = match profile::get_tog_pr(prof_dir, PathBuf::from(prof_path)) {
        Ok(profile) => profile,
        Err(error) => panic!(error),
    };

    let cfg: Cfg = Cfg { profile };

    cfg
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
