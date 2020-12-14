use std::sync::{Arc, Mutex};

use crate::arpx::Arpx;
use crate::error;
use crate::process::{join_and_handle_blocking, Process};
use crate::util::log::{logger, AnnotatedMessage};
use crate::APPNAME;

pub const BUILTINS: &[&str] = &["exit", "exit_error", "kill", "respawn", "silence"];

pub fn act(
    arpx_ref: &mut Arpx,
    action: String,
    pid: String,
    process: Arc<Mutex<Process>>,
    process_name: String,
) -> Result<(), error::ArpxError> {
    match &action[..] {
        "exit" => {
            logger(AnnotatedMessage::new(APPNAME, "Exiting."));
            std::process::exit(0);
        }
        "exit_error" => {
            logger(AnnotatedMessage::new(APPNAME, "Exiting with error."));
            std::process::exit(1);
        }
        "kill" => {
            process.lock().unwrap().child.kill().expect("!kill");
            logger(AnnotatedMessage::new(
                APPNAME,
                &format!("Process {} [pid: {}] killed.", process_name, pid),
            ));
        }
        "respawn" => {
            process.lock().unwrap().kill();
            logger(AnnotatedMessage::new(
                APPNAME,
                &format!(
                    "Process {} [pid: {}] killed. Respawning.",
                    process_name, pid
                ),
            ));

            let process_tuple = match arpx_ref.run_process(process_name) {
                Ok(process_tuple) => process_tuple,
                Err(error) => return Err(error),
            };

            join_and_handle_blocking(process_tuple)?
        }
        "silence" => (),
        _ => (),
    }

    Ok(())
}
