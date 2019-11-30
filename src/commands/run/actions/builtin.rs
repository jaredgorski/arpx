use crate::profile::Profile;
use crate::commands::run;
use crate::commands::run::processes::Process;
use crate::util::log::{log_trigger_snippet, logger, LogData};
use std::sync::{Arc, Mutex};

pub const BUILTINS: &[&str] = &["exit", "kill", "logger", "respawn", "silence"];

pub fn act(profile: &Profile, proc: &Arc<Mutex<Process>>, log_data: &LogData, action: &str) {
    match action {
        "exit" => {
            log_trigger_snippet(log_data, "exit");
            logger("Exiting arpx.");
            std::process::exit(0);
        }
        "kill" => {
            log_trigger_snippet(log_data, "kill");
            proc.lock().unwrap().child.kill().expect("!kill");
            logger(&format!("Process [pid: {}] killed.", proc.lock().unwrap().child.id()));
        }
        "logger" => {
            if !proc.lock().unwrap().silent {
                let annotated_message = &format!("[{}] {}", proc.lock().unwrap().name, log_data.message);
                logger(annotated_message);
            }
        }
        "respawn" => {
            log_trigger_snippet(log_data, "respawn");
            proc.lock().unwrap().child.kill().expect("!kill");
            logger(&format!(
                "Process [{} | pid: {}] killed. Respawning.",
                proc.lock().unwrap().name,
                proc.lock().unwrap().child.id()
            ));
            run::run(&profile, vec![proc.lock().unwrap().name[..].to_string()]);
        }
        "silence" => {}
        _ => {}
    }
}
