use crate::processes::{Process};
use crate::util::log::{LogData};
use crate::config::Cfg;

pub fn handle_action(cfg: &Cfg, proc: &mut Process, log_data: &LogData, exec_actions: Vec<String>) {
    println!("Actions triggered: {:?}", exec_actions);
}
