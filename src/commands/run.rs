use std::collections::HashMap;
use crate::config::Cfg;
use crate::config::profile::ProcessCfg;
use crate::processes::Process;
use crate::handlers::output;

pub fn run(cfg: Cfg, processes: Vec<String>) {
    let profile_processes_map = get_profile_processes_map(cfg);
    let mut procs: Vec<Process> = Vec::new();

    for process in processes {
        let proc_cfg = profile_processes_map.get(&process).expect("Internal process does not match any profile process.");
        procs.push(Process::init(proc_cfg.name[..].to_string(), &proc_cfg.cwd[..], &proc_cfg.command[..]));
    }

    output::handle_output(procs)
}

fn get_profile_processes_map(cfg: Cfg) -> HashMap<String, ProcessCfg> {
    let mut profile_processes: HashMap<String, ProcessCfg> = HashMap::new();
    
    for profile_process in cfg.profile.processes {
        profile_processes.insert(profile_process.name[..].to_string(), profile_process.clone());
    }

    return profile_processes;
}
