use crate::profile::{ActionCfg, ProcessCfg};
use crate::profile::Profile;
use crate::commands::run;
use crate::commands::run::processes::Process;
use crate::util::log::LogData;
use std::io::Write;

pub fn act(profile: &Profile, proc: &mut Process, _log_data: &LogData, action: &str) {
    let exec_action = profile.actions.iter().find(|x| x.name == action);

    if let Some(to_exec) = exec_action {
        match &to_exec.r#type[..] {
            "shell" => exec_shell_type(profile, proc, to_exec),
            _ => (),
        }
    }
}

fn exec_shell_type(profile: &Profile, proc: &mut Process, action: &ActionCfg) {
    if !action.stdin.is_empty() {
        let mut stdin_pipe = proc.child.stdin.take().expect("!stdin");
        let to_write = action.stdin[..].to_string();
        stdin_pipe.write_all(to_write.as_bytes()).expect("!write");
    }

    if !action.command.is_empty() {
        let proc_cfg = ProcessCfg {
            name: action.name[..].to_string(),
            command: action.command[..].to_string(),
            cwd: action.cwd[..].to_string(),
            silent: action.silent,
            blocking: action.blocking,
        };

        run::run_individual(&profile, proc_cfg);
    }
}
