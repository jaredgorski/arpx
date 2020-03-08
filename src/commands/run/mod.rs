use crate::commands::run::handlers::output;
use crate::commands::run::processes::Process;
use crate::profile::{ProcessCfg, Profile};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub mod actions;
pub mod handlers;
pub mod processes;

pub fn run(profile: &Profile, processes: Vec<String>) {
    let profile_processes_map = get_profile_processes_map(&profile);
    let mut proc_handles = Vec::new();

    if processes[0].is_empty() {
        panic!("!processes");
    }

    for process in processes {
        let proc_cfg = profile_processes_map
            .get(&process)
            .expect("Internal process does not match any profile process.");
        let proc = Process::init(
            proc_cfg.name[..].to_string(),
            &proc_cfg.cwd[..],
            &proc_cfg.command[..],
            proc_cfg.silent,
            proc_cfg.blocking,
            proc_cfg.onfail[..].to_string(),
            proc_cfg.onsucceed[..].to_string(),
        );
        let proc2 = Arc::clone(&proc);
        let profile_copy = profile.clone();

        let handle = thread::Builder::new()
            .name(proc_cfg.name[..].to_string())
            .spawn(move || {
                output::handle_output(&profile_copy, &proc);
            })
            .expect("Could not spawn process thread");

        if proc_cfg.blocking {
            handle.join().expect("!join");
            proc2.lock().unwrap().child.wait().expect("!wait");
        } else {
            proc_handles.push(handle);
        }
    }

    // Consider implementing a Join On Drop here
    // --> https://matklad.github.io/2019/08/23/join-your-threads.html

    for handle in proc_handles {
        handle.join().expect("!join");
    }
}

pub fn run_individual(profile: &Profile, proc_cfg: ProcessCfg) {
    let proc = Process::init(
        proc_cfg.name[..].to_string(),
        &proc_cfg.cwd[..],
        &proc_cfg.command[..],
        proc_cfg.silent,
        proc_cfg.blocking,
        proc_cfg.onfail,
        proc_cfg.onsucceed,
    );
    let proc2 = Arc::clone(&proc);
    let profile_copy = profile.clone();
    let handle = thread::Builder::new()
        .name(proc_cfg.name[..].to_string())
        .spawn(move || {
            output::handle_output(&profile_copy, &proc);
        })
        .expect("Could not spawn process thread");

    handle.join().expect("!join");

    if proc_cfg.blocking {
        proc2.lock().unwrap().child.wait().expect("!wait");
    }
}

fn get_profile_processes_map(profile: &Profile) -> HashMap<String, ProcessCfg> {
    let mut profile_processes: HashMap<String, ProcessCfg> = HashMap::new();

    for profile_process in &profile.processes {
        profile_processes.insert(
            profile_process.name[..].to_string(),
            profile_process.clone(),
        );
    }

    profile_processes
}
