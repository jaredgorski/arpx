use std::sync::{Arc, Mutex};

use crate::arpx::Arpx;
use crate::process::{join_and_handle_blocking, Process};
use crate::util::log::logger;

pub const BUILTINS: &[&str] = &["exit", "kill", "respawn", "silence"];

pub fn act(
    arpx_ref: &mut Arpx,
    action: String,
    pid: String,
    process: Arc<Mutex<Process>>,
    process_name: String,
) {
    match &action[..] {
        "exit" => {
            logger("[arpx] Exiting.");
            std::process::exit(0);
        }
        "kill" => {
            process.lock().unwrap().child.kill().expect("!kill");
            logger(&format!("[arpx] Process [pid: {}] killed.", pid));
        }
        "respawn" => {
            let message = format!(
                "[arpx] Process [{} | pid: {}] killed. Respawning.",
                &process_name, pid,
            );
            process.lock().unwrap().kill();
            logger(&message);

            join_and_handle_blocking(arpx_ref.run_process(process_name));
        }
        "silence" => (),
        _ => (),
    }
}
