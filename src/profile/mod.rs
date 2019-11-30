use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::fs;
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

fn default_cwd() -> String {
    ".".to_string()
}

fn default_empty_string() -> String {
    "".to_string()
}

fn default_false() -> bool {
    false
}

fn default_empty_vec_string() -> Vec<String> {
    Vec::new()
}

pub fn get_profile(pathstr: String) -> Result<Profile, Error> {
    let mut path: PathBuf = PathBuf::from(pathstr);

    if path.file_name() == None {
        path.set_file_name("arpx");
        path.set_extension("yaml");
    } else {
        let path_string = path
            .clone()
            .into_os_string()
            .into_string()
            .expect("!string");

        if !path_string.contains("arpx.yaml") {
            path.set_extension("arpx.yaml");
        }
    }

    let pr_file_str = fs::read_to_string(path).expect("Problem reading profile");

    serde_yaml::from_str(&pr_file_str)
}
