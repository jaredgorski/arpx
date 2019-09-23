use std::env;
use crate::config;
use crate::util;

pub mod run;
pub mod setup;

#[derive(Debug)]
pub struct Command {
    pub mode: String,
    pub processes: Vec<String>,
}

pub fn command_parse() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let init_cmd: &str = &args[0][..];

    let mut cfg: config::Cfg = config::Cfg::new();
    let mut cmd_mode: String = String::new();
    let mut cmd_processes: Vec<String> = Vec::new();

    match args.len() - 1 {
        0 => {
            cfg = config::get_sym_cfg("default");
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
                    cfg = config::get_sym_cfg("default");
                    cmd_mode = "run".to_string();
                    cmd_processes.push(value);
                },
                "--profile" | "-f" => {
                    cfg = config::get_sym_cfg(&value);
                    cmd_mode = "run".to_string();

                    for process in cfg.profile.processes.iter() {
                        cmd_processes.push(process.name[..].to_string());
                    }
                },
                "--set-profile" | "-s" => println!("Set profile flag invoked"),
                _ => util::log::usage(init_cmd, Some(&args)),
            }
        },
        4 => {
            let flag1: &str = &args[1][..];
            let value1: String = args[2][..].to_string();
            let flag2: &str = &args[3][..];
            let value2: String = args[4][..].to_string();
            let mut flag1was = "";

            match &flag1[..] {
                "--process" | "-p" => {
                    cmd_mode = "run".to_string();
                    cmd_processes.push(value1[..].to_string());
                    flag1was = "p";
                },
                "--profile" | "-f" => {
                    cfg = config::get_sym_cfg(&value1);
                    cmd_mode = "run".to_string();
                    flag1was = "f";
                },
                _ => util::log::usage(init_cmd, Some(&args)),
            }

            match &flag2[..] {
                "--process" | "-p" => {
                    if flag1was == "p" {
                        panic!("same flag twice");
                    }

                    cmd_mode = "run".to_string();
                    cmd_processes.push(value2[..].to_string());
                },
                "--profile" | "-f" => {
                    if flag1was == "f" {
                        panic!("same flag twice");
                    }

                    cfg = config::get_sym_cfg(&value2);
                    cmd_mode = "run".to_string();
                },
                _ => util::log::usage(init_cmd, Some(&args)),
            }
        },
        _ => util::log::usage(init_cmd, Some(&args)),
    };

    let command: Command = Command {
        mode: cmd_mode,
        processes: cmd_processes,
    };

    run::run(&cfg, command.processes);
}
