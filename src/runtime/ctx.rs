use crate::runtime::{job::task::process::Process, local_bin::BinCommand};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ctx {
    pub process_lib: HashMap<String, Process>,
    pub bin_command: BinCommand,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            process_lib: HashMap::new(),
            bin_command: BinCommand::system_default(),
        }
    }
}
