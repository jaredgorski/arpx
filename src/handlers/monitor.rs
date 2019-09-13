use std::collections::{HashMap};
use crate::config::profile::{MonitorCfg};
use crate::processes::{Process};
use crate::util::log::{LogData};
use crate::config::Cfg;
use crate::handlers::{action, triggers};

#[derive(Debug)]
pub struct MonitorOutput {
    pub exec_actions: Vec<String>,
    pub snippets: HashMap<String, String>,
}

pub fn handle_monitor(cfg: &Cfg, proc: &mut Process, log_data: &mut LogData) {
    let mut exec_actions: Vec<String> = Vec::new();
    let profile_monitors = get_profile_monitors(&cfg);

    for monitor in profile_monitors {
        if monitor.process == proc.name {
            let logs_potential_pull: MonitorOutput = triggers::logs::logs_potential_pull(&monitor.actions, &monitor.triggers.logs, &log_data);
            exec_actions.extend(logs_potential_pull.exec_actions);
            log_data.snippets.extend(logs_potential_pull.snippets);
        }
    }

    action::handle_action(cfg, proc, log_data, exec_actions);
}

fn get_profile_monitors(cfg: &Cfg) -> Vec<&MonitorCfg> {
    let mut profile_monitors: Vec<&MonitorCfg> = Vec::new();
    
    for profile_monitor in &cfg.profile.monitors {
        profile_monitors.push(profile_monitor);
    }

    return profile_monitors;
}

