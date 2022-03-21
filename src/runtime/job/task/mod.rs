pub mod action;
pub mod log_monitor;
pub mod process;

use crate::runtime::ctx::Ctx;
use action::{get_log_monitor_action, get_process_actions};
use anyhow::{bail, Error, Result};
use log::debug;
use process::Process;
use std::thread;

/// Represents and contains a given runtime job task.
///
/// This object contains a list of processes which are executed in order when the task is run.
#[derive(Clone, Debug)]
pub struct Task {
    pub processes: Vec<Process>,
}

impl Task {
    /// Constructs a new, empty `Task`.
    pub fn new(processes: Vec<Process>) -> Self {
        Self { processes }
    }

    /// Executes defined processes in order.
    pub fn run(self, ctx: &Ctx) -> Result<()> {
        debug!("Running task instance with structure:\n{:#?}", self);

        let mut thread_handles = Vec::new();

        for process in self.processes {
            let mut log_monitor_senders = Vec::new();
            for log_monitor_name in &process.log_monitors {
                let log_monitor = &ctx.log_monitor_map[log_monitor_name];

                let log_monitor_action = get_log_monitor_action(log_monitor, ctx);
                let (handle, sender) = log_monitor.clone().run(log_monitor_action)?;

                thread_handles.push(handle);
                log_monitor_senders.push(sender);
            }

            let cloned_ctx = ctx.clone();
            let process_handle = thread::Builder::new()
                .name(process.name.clone())
                .spawn(move || {
                    debug!("Spawned thread \"{}\"", process.name);

                    let process_actions = get_process_actions(&process, &cloned_ctx);
                    process
                        .run(process_actions, &cloned_ctx, &log_monitor_senders)
                        .ok();

                    debug!("Closing thread \"{}\"", process.name);
                })
                .map_err(Error::new)?;

            thread_handles.push(process_handle);
        }

        for handle in thread_handles {
            if handle.join().is_err() {
                bail!("Error joining thread handle");
            }
        }

        Ok(())
    }
}
