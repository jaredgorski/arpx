pub mod message;
pub mod rolling_buffer;

use crate::runtime::{ctx::Ctx, job::task::action::OptionalAction};
use anyhow::{Context, Result};
use crossbeam_channel::{unbounded, Sender};
use log::debug;
use message::{LogMonitorCmd, LogMonitorMessage};
use rolling_buffer::RollingBuffer;
use std::process::{Command, Stdio};
use std::thread;

#[derive(Clone, Debug)]
pub struct LogMonitor {
    pub buffer: RollingBuffer,
    pub buffer_size: usize,
    pub ctx: Ctx,
    pub name: String,
    pub ontrigger: String,
    pub test: String,
}

impl LogMonitor {
    pub fn new(name: String) -> Self {
        Self {
            buffer: RollingBuffer::new(20),
            buffer_size: 20,
            ctx: Ctx::new(),
            name,
            ontrigger: String::new(),
            test: String::new(),
        }
    }

    pub fn buffer_size(mut self, b: usize) -> Self {
        self.buffer = RollingBuffer::new(b);
        self.buffer_size = b;

        self
    }

    pub fn ontrigger(mut self, o: String) -> Self {
        self.ontrigger = o;

        self
    }

    pub fn test(mut self, t: String) -> Self {
        self.test = t;

        self
    }

    pub fn run(
        mut self,
        ontrigger: OptionalAction,
    ) -> Result<(thread::JoinHandle<()>, Sender<LogMonitorMessage>)> {
        debug!("Running log_monitor instance with structure:\n{:#?}", self);

        let name = self.name.clone();

        let (sender, receiver) = unbounded::<LogMonitorMessage>();

        let handle = thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                debug!("Spawned log_monitor thread \"{}\"", &name);

                loop {
                    if let Ok(LogMonitorMessage { cmd, message }) = receiver.recv() {
                        debug!("Received message: {:?}", message);

                        match cmd {
                            LogMonitorCmd::Close => {
                                debug!("Received close message.");
                                break;
                            }
                            LogMonitorCmd::Log => {
                                self.push(message, &ontrigger);
                            }
                            LogMonitorCmd::None => debug!("Received empty message."),
                        }
                    }
                }

                debug!("Closing log_monitor thread \"{}\"", &name);
            })
            .context("Error spawning log monitor thread")?;

        Ok((handle, sender))
    }

    pub fn push(&mut self, line: String, ontrigger: &OptionalAction) {
        self.buffer.push(line);
        self.exec_test(ontrigger).ok();
    }

    pub fn exec_test(&self, ontrigger: &OptionalAction) -> Result<()> {
        let bin = self.ctx.bin_command.bin.clone();
        let mut bin_args = self.ctx.bin_command.args.clone();
        bin_args.push(self.test.clone());

        let status = Command::new(bin)
            .args(bin_args)
            .env("ARPX_BUFFER", &self.buffer.dump()[..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(format!(
                "Error spawning test command on log monitor \"{}\"",
                self.name
            ))?
            .wait()
            .context(format!(
                "Error waiting for test command child on log monitor \"{}\"",
                self.name
            ))?;

        if status.success() {
            debug!("LogMonitor {} triggered", self.name);

            if let Some(action) = ontrigger {
                debug!(
                    "Running ontrigger \"{}\" from prepared action",
                    self.ontrigger
                );

                action();
            }
        }

        Ok(())
    }
}
