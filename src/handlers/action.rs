use std::collections::HashSet;
use crate::processes::{Process};
use crate::util::log::{LogData};
use crate::config::Cfg;
use crate::actions::act;

pub fn handle_action(cfg: &Cfg, proc: &mut Process, log_data: &LogData, exec_actions: &mut Vec<String>) {
    let mut action_set = HashSet::new();

    if !exec_actions.contains(&"silent".to_string()) {
        action_set.insert("logger".to_string());
    }

    exec_actions.retain(|x| action_set.insert(x.clone()));

    for action in action_set {
        act(cfg, proc, log_data, &action[..]);
    }
}
