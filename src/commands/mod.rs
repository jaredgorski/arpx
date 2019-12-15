use crate::profile::{get_profile, Profile};
use clap::ArgMatches;

pub mod run;

#[derive(Debug)]
pub struct Command {
    pub profile: Profile,
    pub daemon_mode: bool,
    pub processes_to_run: Vec<String>,
    pub verbosity: String,
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

    let mut cmd_daemon: bool = false;
    if matches.is_present("daemon") {
        cmd_daemon = true;
        print!("\nNOTE: Daemon mode unstable. Feature not fully implemented yet.\n\n");
    }

    let cmd_verbosity = match matches.occurrences_of("v") {
        0 => "info".to_string(),
        1 => "verbose".to_string(),
        2 => "debug".to_string(),
        3 | _ => "silly".to_string(),
    };

    if cmd_verbosity != "info" {
        print!("\nNOTE: Verbosity level is info. Feature not implemented yet.\n\n");
    }

    Command {
        profile: cmd_profile,
        daemon_mode: cmd_daemon,
        processes_to_run: cmd_processes,
        verbosity: cmd_verbosity,
    }
}
