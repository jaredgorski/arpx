use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    #[serde(default = "default_processes")]
    pub processes: Vec<ProcessCfg>,
    #[serde(default = "default_monitors")]
    pub monitors: Vec<MonitorCfg>,
    #[serde(default = "default_actions")]
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
    pub daemon: bool,
    #[serde(default = "default_false")]
    pub silent: bool,
    #[serde(default = "default_false")]
    pub blocking: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonitorCfg {
    #[serde(default = "default_empty_string")]
    pub process: String,
    #[serde(default = "default_empty_string")]
    pub condition: String,
    #[serde(default = "default_empty_vec_string")]
    pub actions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionCfg {
    #[serde(default = "default_empty_string")]
    pub name: String,
    #[serde(default = "default_empty_string")]
    pub command: String,
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default = "default_false")]
    pub daemon: bool,
    #[serde(default = "default_shell")]
    pub r#type: String,
    #[serde(default = "default_empty_string")]
    pub stdin: String,
    #[serde(default = "default_false")]
    pub silent: bool,
    #[serde(default = "default_false")]
    pub blocking: bool,
}

fn default_processes() -> Vec<ProcessCfg> {
    vec![ProcessCfg {
        name: default_empty_string(),
        command: default_empty_string(),
        cwd: default_empty_string(),
        daemon: default_false(),
        silent: default_false(),
        blocking: default_false(),
    }]
}

fn default_monitors() -> Vec<MonitorCfg> {
    vec![MonitorCfg {
        process: default_empty_string(),
        condition: default_empty_string(),
        actions: default_empty_vec_string(),
    }]
}

fn default_actions() -> Vec<ActionCfg> {
    vec![ActionCfg {
        name: default_empty_string(),
        command: default_empty_string(),
        cwd: default_empty_string(),
        daemon: default_false(),
        r#type: default_empty_string(),
        stdin: default_empty_string(),
        silent: default_false(),
        blocking: default_false(),
    }]
}

fn default_cwd() -> String {
    ".".to_string()
}

fn default_empty_string() -> String {
    "".to_string()
}

fn default_shell() -> String {
    "shell".to_string()
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

