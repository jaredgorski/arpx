use crate::runtime::{
    job::task::{log_monitor::LogMonitor, process::Process},
    local_bin::BinCommand,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Ctx {
    pub bin_command: BinCommand,
    pub log_monitor_lib: HashMap<String, LogMonitor>,
    pub process_lib: HashMap<String, Process>,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            bin_command: BinCommand::system_default(),
            log_monitor_lib: HashMap::new(),
            process_lib: HashMap::new(),
        }
    }
}
