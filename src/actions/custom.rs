use crate::util::log::{LogData};
use crate::processes::{Process};
use crate::config::Cfg;
use crate::config::profile::{ActionCfg, ProcessCfg};
use crate::commands::run;

pub fn act(cfg: &Cfg, _proc: &mut Process, _log_data: &LogData, action: &str) {
    let exec_action = cfg.profile.actions.iter().find(|x| x.name == action);

    match exec_action {
        Some(to_exec) => {
            match &to_exec.r#type[..] {
                "shell" => exec_shell_type(cfg, to_exec),
                _ => (),
            }
        },
        None => (),
    }
}

fn exec_shell_type(cfg: &Cfg, action: &ActionCfg) {
    let proc_cfg = ProcessCfg {
        name: action.name[..].to_string(),
        command: action.command[..].to_string(),
        cwd: ".".to_string(),
        silent: action.silent,
    };

    run::run_individual(&cfg, proc_cfg);
}
