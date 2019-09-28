use crate::cfg::profile::{ActionCfg, ProcessCfg};
use crate::cfg::Cfg;
use crate::commands::run;
use crate::commands::run::processes::Process;
use crate::util::log::LogData;
use std::io::Write;

pub fn act(cfg: &Cfg, proc: &mut Process, _log_data: &LogData, action: &str) {
    let exec_action = cfg.profile.actions.iter().find(|x| x.name == action);

    if let Some(to_exec) = exec_action {
        match &to_exec.r#type[..] {
            "shell" => exec_shell_type(cfg, proc, to_exec),
            _ => (),
        }
    }
}

fn exec_shell_type(cfg: &Cfg, proc: &mut Process, action: &ActionCfg) {
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

        run::run_individual(&cfg, proc_cfg);
    }
}
