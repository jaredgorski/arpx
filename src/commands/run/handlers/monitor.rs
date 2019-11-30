use crate::commands::run::handlers::{action, triggers};
use crate::commands::run::processes::Process;
use crate::profile::{Profile,MonitorCfg};
use crate::util::log::LogData;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct MonitorOutput {
    pub exec_actions: Vec<String>,
    pub snippets: HashMap<String, String>,
}

pub fn handle_monitor(profile: &Profile, proc: &Arc<Mutex<Process>>, log_data: &mut LogData) {
    let mut exec_actions: Vec<String> = Vec::new();
    let profile_monitors = get_profile_monitors(&profile);

    for monitor in profile_monitors {
        if monitor.process == proc.lock().unwrap().name {
            let logs_potential_pull: MonitorOutput = triggers::logs::logs_potential_pull(
                &monitor.actions,
                &monitor.triggers.logs,
                &log_data,
            );
            exec_actions.extend(logs_potential_pull.exec_actions);
            log_data.snippets.extend(logs_potential_pull.snippets);
        }
    }

    action::handle_action(profile, &proc, log_data, &mut exec_actions);
}

fn get_profile_monitors(profile: &Profile) -> Vec<&MonitorCfg> {
    let mut profile_monitors: Vec<&MonitorCfg> = Vec::new();

    for profile_monitor in &profile.monitors {
        profile_monitors.push(profile_monitor);
    }

    profile_monitors
}
