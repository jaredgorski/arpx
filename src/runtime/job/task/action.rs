use crate::runtime::{
    ctx::Ctx,
    job::task::{log_monitor::LogMonitor, process::Process},
};
use log::debug;

pub const BUILTIN_ACTIONS: [&str; 2] = ["arpx_exit", "arpx_exit_error"];

pub struct ProcessActions {
    pub onfail: OptionalAction,
    pub onsucceed: OptionalAction,
}

pub type OptionalAction = Option<Box<dyn Fn() + Send>>;

pub fn get_process_actions(process: &Process, ctx: &Ctx) -> ProcessActions {
    let onfail = match &process.onfail {
        Some(action_name) => get_optional_action(action_name.into(), ctx.clone()),
        None => None,
    };
    let onsucceed = match &process.onsucceed {
        Some(action_name) => get_optional_action(action_name.into(), ctx.clone()),
        None => None,
    };

    ProcessActions { onfail, onsucceed }
}

pub fn get_log_monitor_action(log_monitor: &LogMonitor, ctx: &Ctx) -> OptionalAction {
    get_optional_action(log_monitor.ontrigger.clone(), ctx.clone())
}

fn get_optional_action(action_name: String, ctx: Ctx) -> OptionalAction {
    if BUILTIN_ACTIONS.contains(&&action_name[..]) {
        return Some(Box::new(move || execute_action(&action_name[..])));
    }

    match ctx.clone().process_map.get(&action_name[..]) {
        Some(process) => {
            let cloned_process = process.clone();

            Some(Box::new(move || {
                let process_actions = get_process_actions(&cloned_process, &ctx);

                cloned_process.run(process_actions, &ctx, &[]).ok();
            }))
        }
        None => None,
    }
}

fn execute_action(action: &str) {
    match action {
        "arpx_exit" => {
            debug!("Received builtin action \"arpx_exit\". Exiting runtime.");
            std::process::exit(0)
        }
        "arpx_exit_error" => {
            debug!(
                "Received builtin action \"arpx_exit_error\". Exiting runtime with error status."
            );
            std::process::exit(1)
        }
        _ => {
            debug!("Unknown builtin action received. Doing nothing.");
        }
    }
}
