use crate::profile::{get_tog_pr, Profile};
use clap::{ArgMatches};

pub mod run;

#[derive(Debug)]
pub struct Command {
    pub profile: Profile,
    pub daemon_mode: bool,
    pub pid_to_kill: String,
    pub processes_to_run: Vec<String>,
    pub verbosity: String,
}

pub fn get_command(matches: ArgMatches) -> Command {
    let mut cmd_profile: Profile = Profile::new();
    let mut cmd_daemon: bool = false;
    let mut cmd_kill: String = String::new();
    let mut cmd_processes: Vec<String> = Vec::new();

    if matches.is_present("ls") {
        println!("NOTE: Processes cannot be listed. Feature not fully implemented yet.");
    } else {
        let file: String = matches.value_of("file").unwrap_or("tog.yaml").to_string();

        cmd_profile = match get_tog_pr(file) {
            Ok(profile) => profile,
            Err(error) => panic!(error),
        };

        if matches.is_present("daemon") {
            cmd_daemon = true;
            println!("NOTE: Daemon mode not invoked. Feature not fully implemented yet.");
        }

        if let Some(pid) = matches.value_of("kill") {
            cmd_kill = pid.to_string();
            println!("NOTE: Process cannot be killed. Feature not fully implemented yet.");
        }

        if let Some(ref process) = matches.value_of("process") {
            cmd_processes.push(process.to_string());
        } else {
            for process in cmd_profile.processes.iter() {
                cmd_processes.push(process.name[..].to_string());
            }
        }
    }

    let cmd_verbosity = match matches.occurrences_of("v") {
        0 => "info".to_string(),
        1 => "verbose".to_string(),
        2 => "debug".to_string(),
        3 | _ => "silly".to_string(),
    };

    if cmd_verbosity != "info" {
        println!("NOTE: Verbosity level is info. Feature not fully implemented yet.");
    }

    Command {
        profile: cmd_profile,
        daemon_mode: cmd_daemon,
        pid_to_kill: cmd_kill,
        processes_to_run: cmd_processes,
        verbosity: cmd_verbosity,
    }
}
