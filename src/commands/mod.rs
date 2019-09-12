use std::env;
use crate::config;
use crate::util;

pub mod run;

#[derive(Debug)]
pub struct Command {
    pub mode: String,
    pub processes: Vec<String>,
}

pub fn command_parse() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let init_cmd: &str = &args[0][..];

    let cfg = config::get_tom_cfg();

    let mut cmd_mode: String = String::new();
    let mut cmd_processes: Vec<String> = Vec::new();

    match args.len() - 1 {
        0 => {
            cmd_mode = "run".to_string();

            for process in cfg.profile.processes.iter() {
                cmd_processes.push(process.name[..].to_string());
            }
        },
        1 => {
            match &args[1][..] {
                "--help" | "-h" => util::log::usage(init_cmd, None),
                _ => util::log::usage(init_cmd, Some(&args)),
            }
        },
        2 => {
            let flag: &str = &args[1][..];
            let value: String = args[2][..].to_string();

            match &flag[..] {
                "--process" | "-p" => {
                    cmd_mode = "run".to_string();
                    cmd_processes.push(value);
                },
                "--set-profile" => println!("Set profile flag invoked"),
                _ => util::log::usage(init_cmd, Some(&args)),
            }

        },
        _ => util::log::usage(init_cmd, Some(&args)),
    };

    let command: Command = Command {
        mode: cmd_mode,
        processes: cmd_processes,
    };

    run::run(cfg, command.processes);
}
