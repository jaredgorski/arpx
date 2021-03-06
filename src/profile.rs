use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{self, ArpxError};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    #[serde(default = "default_empty_string")]
    pub entrypoint: String,
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
            entrypoint: String::new(),
            processes: Vec::new(),
            monitors: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn from_file(pathstr: String) -> Result<Profile, ArpxError> {
        let mut path: PathBuf = PathBuf::from(pathstr[..].to_string());

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

        let pr_file_str = match fs::read_to_string(path) {
            Ok(file_str) => file_str,
            Err(_) => return Err(error::profile_not_found(pathstr)),
        };

        match serde_yaml::from_str(&pr_file_str) {
            Ok(deserialized) => Ok(deserialized),
            Err(error) => Err(error::profile_parse_error(format!("{}", error))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessCfg {
    #[serde(default = "default_empty_string")]
    pub name: String,
    #[serde(default = "default_empty_string")]
    pub color: String,
    #[serde(default = "default_empty_string")]
    pub command: String,
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default = "default_false")]
    pub silent: bool,
    #[serde(default = "default_false")]
    pub blocking: bool,
    #[serde(default = "default_empty_string")]
    pub onfail: String,
    #[serde(default = "default_empty_string")]
    pub onsucceed: String,
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
    #[serde(default = "default_empty_string")]
    pub color: String,
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default = "default_shell")]
    pub r#type: String,
    #[serde(default = "default_empty_string")]
    pub stdin: String,
    #[serde(default = "default_false")]
    pub silent: bool,
    #[serde(default = "default_false")]
    pub blocking: bool,
    #[serde(default = "default_empty_string")]
    pub onfail: String,
    #[serde(default = "default_empty_string")]
    pub onsucceed: String,
}

fn default_processes() -> Vec<ProcessCfg> {
    vec![ProcessCfg {
        name: default_empty_string(),
        command: default_empty_string(),
        color: default_empty_string(),
        cwd: default_empty_string(),
        silent: default_false(),
        blocking: default_false(),
        onfail: default_empty_string(),
        onsucceed: default_empty_string(),
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
        color: default_empty_string(),
        cwd: default_empty_string(),
        r#type: default_empty_string(),
        stdin: default_empty_string(),
        silent: default_false(),
        blocking: default_false(),
        onfail: default_empty_string(),
        onsucceed: default_empty_string(),
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
