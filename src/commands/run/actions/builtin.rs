use crate::cfg::Cfg;
use crate::commands::run;
use crate::commands::run::processes::Process;
use crate::util::log::{log_trigger_snippet, logger, LogData};

pub const BUILTINS: &[&str] = &["kill", "logger", "respawn", "silence", "togexit"];

pub fn act(cfg: &Cfg, proc: &mut Process, log_data: &LogData, action: &str) {
    match action {
        "kill" => {
            log_trigger_snippet(log_data, "kill");
            proc.child.kill().expect("!kill");
            logger(&format!("Process [pid: {}] killed.", proc.child.id()));
        }
        "logger" => {
            if !proc.silent {
                let annotated_message = &format!("[{}] {}", proc.name, log_data.message);
                logger(annotated_message);
            }
        }
        "respawn" => {
            log_trigger_snippet(log_data, "respawn");
            proc.child.kill().expect("!kill");
            logger(&format!(
                "Process [{} | pid: {}] killed. Respawning.",
                proc.name,
                proc.child.id()
            ));
            run::run(&cfg, vec![proc.name[..].to_string()]);
        }
        "silence" => {}
        "togexit" => {
            log_trigger_snippet(log_data, "togexit");
            logger("Exiting tog.");
            std::process::exit(0);
        }
        _ => {}
    }
}
