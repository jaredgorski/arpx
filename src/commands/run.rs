use std::collections::HashMap;
use std::thread;
use crate::config::Cfg;
use crate::config::profile::ProcessCfg;
use crate::processes::Process;
use crate::handlers::output;

pub fn run(cfg: &Cfg, processes: Vec<String>) {
    let profile_processes_map = get_profile_processes_map(&cfg);
    let mut proc_handles = Vec::new();

    for process in processes {
        let proc_cfg = profile_processes_map.get(&process).expect("Internal process does not match any profile process.");
        let proc = Process::init(proc_cfg.name[..].to_string(), &proc_cfg.cwd[..], &proc_cfg.command[..], &proc_cfg.silent);
        let cfg_copy = cfg.clone();
        let handle = thread::Builder::new()
            .name(proc_cfg.name[..].to_string().into())
            .spawn(move || {
                output::handle_output(&cfg_copy, proc);
            }).expect("Could not spawn process thread");
        proc_handles.push(handle);
    }

    for handle in proc_handles {
        handle.join().expect("!join");
    }
}

pub fn run_individual(cfg: &Cfg, proc_cfg: ProcessCfg) {
    let proc = Process::init(proc_cfg.name[..].to_string(), &proc_cfg.cwd[..], &proc_cfg.command[..], &proc_cfg.silent);
    let cfg_copy = cfg.clone();
    let handle = thread::Builder::new()
        .name(proc_cfg.name[..].to_string().into())
        .spawn(move || {
            output::handle_output(&cfg_copy, proc);
        }).expect("Could not spawn process thread");

    handle.join().expect("!join");
}

fn get_profile_processes_map(cfg: &Cfg) -> HashMap<String, ProcessCfg> {
    let mut profile_processes: HashMap<String, ProcessCfg> = HashMap::new();
    
    for profile_process in &cfg.profile.processes {
        profile_processes.insert(profile_process.name[..].to_string(), profile_process.clone());
    }

    return profile_processes;
}
