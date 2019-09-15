use crate::commands::run::processes::{Process};
use crate::util::log::{LogData};
use crate::config::Cfg;

pub mod builtin;
pub mod custom;

pub fn act(cfg: &Cfg, proc: &mut Process, log_data: &LogData, action: &str) {
    if builtin::BUILTINS.contains(&action) {
        builtin::act(cfg, proc, log_data, &action[..]);
    } else {
        custom::act(cfg, proc, log_data, &action[..]);
    }
}
