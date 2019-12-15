use crate::commands::run::actions::act;
use crate::commands::run::processes::Process;
use crate::profile::Profile;
use crate::util::log::LogData;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn handle_action(
    profile: &Profile,
    proc: &Arc<Mutex<Process>>,
    log_data: &LogData,
    exec_actions: &mut Vec<String>,
) {
    let mut action_set = HashSet::new();

    if !exec_actions.contains(&"silence".to_string()) {
        action_set.insert("logger".to_string());
    }

    exec_actions.retain(|x| action_set.insert(x.clone()));

    for action in action_set {
        act(profile, &proc, log_data, &action[..]);
    }
}
