use crate::runtime::{
    job::task::{log_monitor::LogMonitor, process::Process},
    local_bin::BinCommand,
};
use std::collections::HashMap;

/// Runtime context object.
///
/// This object contains indexes to defined processes and log monitors as well as the current
/// `BinCommand` object.
#[derive(Clone, Debug)]
pub struct Ctx {
    pub bin_command: BinCommand,
    pub log_monitor_map: HashMap<String, LogMonitor>,
    pub process_map: HashMap<String, Process>,
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
    }
}

impl Ctx {
    /// Constructs a new, empty `Ctx`.
    pub fn new() -> Self {
        Self {
            bin_command: BinCommand::system_default(),
            log_monitor_map: HashMap::new(),
            process_map: HashMap::new(),
        }
    }
}
