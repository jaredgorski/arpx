use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::arpx::Arpx;
use crate::process::{join_and_handle_blocking, Process};
use crate::profile::ProcessCfg;

pub fn act(
    arpx_ref: &mut Arpx,
    action: String,
    _pid: String,
    process: Arc<Mutex<Process>>,
    _process_name: String,
) {
    let exec_action = arpx_ref.profile.actions.iter().find(|x| x.name == action);

    if let Some(action) = exec_action {
        match &action.r#type[..] {
            "shell" => {
                if !action.stdin.is_empty() {
                    let mut stdin_pipe =
                        process.lock().unwrap().child.stdin.take().expect("!stdin");
                    let to_write = action.stdin[..].to_string();
                    stdin_pipe.write_all(to_write.as_bytes()).expect("!write");
                }

                if !action.command.is_empty() {
                    let process_cfg = ProcessCfg {
                        name: action.name[..].to_string(),
                        command: action.command[..].to_string(),
                        color: action.color[..].to_string(),
                        cwd: action.cwd[..].to_string(),
                        silent: action.silent,
                        blocking: action.blocking,
                        onfail: action.onfail[..].to_string(),
                        onsucceed: action.onsucceed[..].to_string(),
                    };

                    join_and_handle_blocking(arpx_ref.run_process_from_cfg(&process_cfg));
                }
            }
            _ => (),
        }
    }
}
