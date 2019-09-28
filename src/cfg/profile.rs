use crate::cfg::{default_cwd, default_empty_string, default_empty_vec_string, default_false};
use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    pub processes: Vec<ProcessCfg>,
    pub monitors: Vec<MonitorCfg>,
    pub actions: Vec<ActionCfg>,
}

impl Profile {
    pub fn new() -> Profile {
        Profile {
            processes: Vec::new(),
            monitors: Vec::new(),
            actions: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessCfg {
    #[serde(default = "default_empty_string")]
    pub name: String,
    #[serde(default = "default_empty_string")]
    pub command: String,
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default = "default_false")]
    pub silent: bool,
    #[serde(default = "default_false")]
    pub blocking: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonitorCfg {
    #[serde(default = "default_empty_string")]
    pub process: String,
    pub triggers: Triggers,
    #[serde(default = "default_empty_vec_string")]
    pub actions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Triggers {
    pub logs: LogTriggerCfg,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogTriggerCfg {
    #[serde(default = "default_empty_string")]
    pub includes_string: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionCfg {
    #[serde(default = "default_empty_string")]
    pub name: String,
    #[serde(default = "default_empty_string")]
    pub command: String,
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default = "default_empty_string")]
    pub r#type: String,
    #[serde(default = "default_empty_string")]
    pub stdin: String,
    #[serde(default = "default_false")]
    pub silent: bool,
    #[serde(default = "default_false")]
    pub blocking: bool,
}

pub fn get_tog_pr(prof_dir: PathBuf, mut path: PathBuf) -> Result<Profile, Error> {
    let path_string = path
        .clone()
        .into_os_string()
        .into_string()
        .expect("!string");

    if !path_string.contains("tog.yaml") {
        path.set_extension("tog.yaml");
    }

    let pr_path: PathBuf = [prof_dir, path].iter().collect();
    let mut pr_file: File = File::open(pr_path).expect("Unable to open profile file");

    let mut pr_file_str = String::new();
    pr_file
        .read_to_string(&mut pr_file_str)
        .expect("Unable to read profile file");
    let profile: Result<Profile, Error> = serde_yaml::from_str(&pr_file_str);

    profile
}
