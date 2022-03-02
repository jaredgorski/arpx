use crate::runtime::{
    job::task::log_monitor::DEFAULT_VARIABLE_PATTERN,
    profile::deserialize::{log_monitors, processes},
};
use arpx_job_parser::Job;
use std::collections::HashMap;

pub fn jobs() -> HashMap<String, Job> {
    HashMap::new()
}

pub fn log_monitors() -> HashMap<String, log_monitors::LogMonitor> {
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

pub fn string_vec() -> Vec<String> {
    Vec::new()
}

pub fn buffer_size() -> usize {
    20
}

pub fn variable_pattern() -> String {
    DEFAULT_VARIABLE_PATTERN.to_string()
}
