use crate::runtime::profile::deserialize::processes;
use arpx_job_parser::Job;
use std::collections::HashMap;

pub fn jobs() -> HashMap<String, Job> {
    HashMap::new()
}

pub fn processes() -> HashMap<String, processes::Process> {
    HashMap::new()
}

pub fn cwd() -> String {
    ".".to_string()
}

pub fn string() -> String {
    "".to_string()
}
