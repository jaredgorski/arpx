pub mod action;
pub mod log_monitor;
pub mod process;

use crate::runtime::ctx::Ctx;
use action::{get_log_monitor_action, get_process_actions};
use log::debug;
use process::Process;
use std::thread;

#[derive(Clone, Debug)]
pub struct Task {
    pub processes: Vec<Process>,
}

impl Task {
    pub fn new(processes: Vec<Process>) -> Self {
        Self { processes }
    }

    pub fn run(self, ctx: &Ctx) -> Result<(), std::io::Error> {
        debug!("Running task instance with structure:\n{:#?}", self);

        let mut thread_handles = Vec::new();

        for process in self.processes {
            let mut log_monitor_senders = Vec::new();
            process.log_monitors.iter().for_each(|log_monitor_name| {
                let log_monitor = &ctx.log_monitor_lib[log_monitor_name];

                let log_monitor_action = get_log_monitor_action(log_monitor, ctx);
                let (handle, sender) = log_monitor.clone().run(log_monitor_action);

                thread_handles.push(handle);
                log_monitor_senders.push(sender);
            });

            let cloned_ctx = ctx.clone();
            let process_handle =
                match thread::Builder::new()
                    .name(process.name.clone())
                    .spawn(move || {
                        debug!("Spawned thread \"{}\"", process.name);

                        let process_actions = get_process_actions(&process, &cloned_ctx);
                        process.run(process_actions, &cloned_ctx, &log_monitor_senders);

                        debug!("Closing thread \"{}\"", process.name);
                    }) {
                    Ok(handle) => handle,
                    Err(error) => return Err(error),
                };

            thread_handles.push(process_handle);
        }

        for handle in thread_handles {
            match handle.join() {
                Ok(()) => (),
                Err(_) => panic!("!join"),
            }
        }

        Ok(())
    }
}
