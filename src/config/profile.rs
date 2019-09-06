use serde::{Serialize, Deserialize};
use serde_yaml::{Error};
use std::fs::{File};
use std::path::{PathBuf};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub processes: Vec<ProcessCfg>,
    pub monitors: Vec<MonitorCfg>,
    pub actions: Vec<ActionCfg>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessCfg {
    pub name: String,
    pub command: String,
    pub cwd: String,
    pub silent: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorCfg {
    pub process: String,
    pub triggers: Triggers,
    pub actions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Triggers {
    pub logs: LogTriggerCfg,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogTriggerCfg {
    pub includes_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionCfg {
    pub name: String,
    pub r#type: String,
    pub command: String,
}

pub fn get_tom_pr(tom_dir: PathBuf, mut path: PathBuf) -> Result<Profile, Error> {
    path.set_extension("tom.yaml");
    let pr_path: PathBuf = [tom_dir, path].iter().collect();
    let mut pr_file: File = File::open(pr_path).expect("Unable to open profile file");

    let mut pr_file_str = String::new(); 
    pr_file.read_to_string(&mut pr_file_str).expect("Unable to read profile file");
    let profile: Result<Profile, Error> = serde_yaml::from_str(&pr_file_str);

    return profile;
}
