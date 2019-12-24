use crate::profile::{get_profile, Profile};
use clap::ArgMatches;

pub mod run;

#[derive(Debug)]
pub struct Command {
    pub profile: Profile,
    pub processes_to_run: Vec<String>,
}

pub fn get_command(matches: ArgMatches) -> Command {
    let file: String = matches.value_of("file").unwrap_or("arpx.yaml").to_string();
    let cmd_profile = match get_profile(file) {
        Ok(profile) => profile,
        Err(error) => panic!(error),
    };

    let mut cmd_processes: Vec<String> = Vec::new();
    if let Some(ref process) = matches.value_of("process") {
        cmd_processes.push(process.to_string());
    } else {
        for process in cmd_profile.processes.iter() {
            cmd_processes.push(process.name[..].to_string());
        }
    }

    Command {
        profile: cmd_profile,
        processes_to_run: cmd_processes,
    }
}
