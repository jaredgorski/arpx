use crate::runtime::{job::task::process::Process, local_bin::BinCommand};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ctx {
    pub bin_command: BinCommand,
    pub process_lib: HashMap<String, Process>,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            bin_command: BinCommand::system_default(),
            process_lib: HashMap::new(),
        }
    }
}
