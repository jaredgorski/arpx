pub mod action;
pub mod process;
mod uplink;

use crate::runtime::ctx::Ctx;
use action::execute_action;
use crossbeam_channel::unbounded;
use log::debug;
use process::Process;
use std::thread;
use uplink::{Cmd, UplinkMessage};

#[derive(Clone, Debug)]
pub struct Task {
    pub processes: Vec<Process>,
}

impl Task {
    pub fn new(processes: Vec<Process>) -> Self {
        Self { processes }
    }

    pub fn run(self, ctx: Ctx) -> Result<(), std::io::Error> {
        debug!("Running task instance with structure:\n{:#?}", self);

        let mut thread_handles = Vec::new();

        let (sender, receiver) = unbounded::<UplinkMessage>();

        for process in self.processes {
            let cloned_ctx = ctx.clone();
            let cloned_sender = sender.clone();

            let handle = match thread::Builder::new()
                .name(process.name.clone())
                .spawn(move || {
                    debug!("Spawned thread \"{}\"", process.name);

                    process.clone().run(cloned_ctx, cloned_sender);

                    debug!("Closing thread \"{}\"", process.name);
                }) {
                Ok(handle) => handle,
                Err(error) => return Err(error),
            };

            thread_handles.push(handle);
        }

        for handle in thread_handles {
            match handle.join() {
                Ok(()) => (),
                Err(_) => panic!("!join"),
            }
        }

        for received in receiver.try_iter().collect::<Vec<UplinkMessage>>() {
            let UplinkMessage { cmd, message } = received;

            debug!("Received uplink message: {:?}", message);

            match cmd {
                Cmd::Action => execute_action(&message[..]),
                Cmd::None => debug!("Received empty uplink message."),
            }
        }

        Ok(())
    }
}
