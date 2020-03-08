use crate::commands::run;
use crate::commands::run::processes::Process;
use crate::profile::Profile;
use crate::util::log::{log_trigger_snippet, logger, LogData};
use std::sync::{Arc, Mutex};

pub const BUILTINS: &[&str] = &["exit", "kill", "logger", "respawn", "silence"];

pub fn act(
    profile: &Profile,
    proc: &Arc<Mutex<Process>>,
    log_data: &LogData,
    action: &str,
    proc_exited: bool,
) {
    match action {
        "exit" => {
            log_trigger_snippet(log_data, "exit");
            logger("[arpx] Exiting.");
            std::process::exit(0);
        }
        "kill" => {
            if proc_exited {
                return;
            }

            log_trigger_snippet(log_data, "kill");
            proc.lock().unwrap().child.kill().expect("!kill");
            logger(&format!(
                "[arpx] Process [pid: {}] killed.",
                proc.lock().unwrap().child.id()
            ));
        }
        "logger" => {
            if !proc.lock().unwrap().silent {
                let annotated_message =
                    &format!("[{}] {}", proc.lock().unwrap().name, log_data.message);
                logger(annotated_message);
            }
        }
        "respawn" => {
            let respawn_proc = proc.lock().unwrap().name[..].to_string();

            if !proc_exited {
                log_trigger_snippet(log_data, "respawn");
                let old_id = proc.lock().unwrap().child.id();
                let message = format!(
                    "[arpx] Process [{} | pid: {}] killed. Respawning.",
                    &respawn_proc, old_id,
                );
                proc.lock().unwrap().child.kill().expect("!kill");
                logger(&message);
            }

            run::run(&profile, vec![respawn_proc]);
        }
        "silence" => (),
        _ => (),
    }
}
