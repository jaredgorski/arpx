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

/// Represents and contains a given runtime job task log monitor.
///
/// This object contains all of the data necessary to run a given log monitor. This data includes
/// the log monitor name, the size of its rolling buffer, the rolling buffer instance itself, the
/// `exec` command which should be executed on each push to the buffer, and the `onsucceed` action
/// which should run if `exec` returns with a `0` exit code.
#[derive(Clone, Debug)]
pub struct LogMonitor {
    pub buffer: RollingBuffer,
    pub buffer_size: usize,
    pub ctx: Ctx,
    pub exec: String,
    pub name: String,
    pub onsucceed: String,
}

impl LogMonitor {
    /// Constructs a new, empty `LogMonitor`.
    pub fn new(name: String) -> Self {
        Self {
            buffer: RollingBuffer::new(20),
            buffer_size: 20,
            ctx: Ctx::new(),
            exec: String::new(),
            name,
            onsucceed: String::new(),
        }
    }

    /// Builds `LogMonitor` with the specified rolling buffer size.
    pub fn buffer_size(mut self, b: usize) -> Self {
        self.buffer = RollingBuffer::new(b);
        self.buffer_size = b;

        self
    }

    /// Builds `LogMonitor` with the name of the action to execute if the `exec` succeeds.
    pub fn onsucceed(mut self, o: String) -> Self {
        self.onsucceed = o;

        self
    }

    /// Builds `LogMonitor` with the specified exec.
    pub fn exec(mut self, t: String) -> Self {
        self.exec = t;

        self
    }

    /// Executes the log monitor using the provided action.
    pub fn run(
        mut self,
        onsucceed: OptionalAction,
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
                                self.push(message, &onsucceed);
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

    /// Pushes a line of text to the rolling buffer and executes the exec command on the new buffer
    /// state.
    pub fn push(&mut self, line: String, onsucceed: &OptionalAction) {
        self.buffer.push(line);
        self.run_exec(onsucceed).ok();
    }

    /// Executes the current exec command and, if successful, performs the `onsucceed` action.
    pub fn run_exec(&self, onsucceed: &OptionalAction) -> Result<()> {
        let bin = self.ctx.bin_command.bin.clone();
        let mut bin_args = self.ctx.bin_command.args.clone();
        bin_args.push(self.exec.clone());

        let status = Command::new(bin)
            .args(bin_args)
            .env("ARPX_BUFFER", &self.buffer.dump()[..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(format!(
                "Error spawning exec command on log monitor \"{}\"",
                self.name
            ))?
            .wait()
            .context(format!(
                "Error waiting for exec command child on log monitor \"{}\"",
                self.name
            ))?;

        if status.success() {
            debug!("LogMonitor {} triggered", self.name);

            if let Some(action) = onsucceed {
                debug!(
                    "Running onsucceed \"{}\" from prepared action",
                    self.onsucceed
                );

                action();
            }
        }

        Ok(())
    }
}
