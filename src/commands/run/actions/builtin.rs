use crate::util::log::{LogData, logger, log_trigger_snippet};
use crate::commands::run::processes::{Process};
use crate::config::Cfg;
use crate::commands::run;

pub const BUILTINS: &[&str] = &[
    "kill",
    "logger",
    "respawn",
    "silence",
    "tomexit",
];

pub fn act(cfg: &Cfg, proc: &mut Process, log_data: &LogData, action: &str) {
    match action {
        "kill" => {
            log_trigger_snippet(log_data, "kill");
            proc.child.kill().expect("!kill");
            logger(&format!("Process [pid: {}] killed.", proc.child.id()));
        },
        "logger" => {
            if !proc.silent {
                logger(log_data.message);
            }
        },
        "respawn" => {
            log_trigger_snippet(log_data, "respawn");
            proc.child.kill().expect("!kill");
            logger(&format!("Process [pid: {}] killed. Respawning...", proc.child.id()));
            run::run(&cfg, vec![proc.name[..].to_string()]);
        },
        "silence" => {},
        "tomexit" => {
            log_trigger_snippet(log_data, "tomexit");
            logger("Exiting tom.");
            std::process::exit(0);
        },
        _ => {},
    }
}
