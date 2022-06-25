mod stream;

use crate::runtime::{
    ctx::Ctx,
    job::task::{
        action::ProcessActions,
        log_monitor::message::{LogMonitorCmd, LogMonitorMessage},
    },
    local_bin::BinCommand,
};
use anyhow::{bail, Context, Result};
use crossbeam_channel::Sender;
use log::{debug, info};
use std::process::{Command, Stdio};
use stream::PipeStreamReader;

#[cfg(windows)]
use std::os::windows::process::CommandExt;


/// Represents and contains a given runtime job task process.
///
/// This object contains all of the data necessary to run a given process. This data includes the
/// process name, the `exec` which should be executed using the current `BinCommand`, the
/// directory in which to execute the `exec`, any log monitors which should monitor the command
/// output, as well as any actions which should be performed when the command fails or succeeds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Process {
    pub cwd: String,
    pub exec: String,
    pub log_monitors: Vec<String>,
    pub name: String,
    pub onfail: Option<String>,
    pub onsucceed: Option<String>,
}

impl Process {
    /// Constructs a new, empty `Process`.
    pub fn new(name: String) -> Self {
        Self {
            cwd: ".".to_owned(),
            exec: String::new(),
            log_monitors: Vec::new(),
            name,
            onfail: None,
            onsucceed: None,
        }
    }

    /// Builds `Process` with the specified command.
    pub fn exec(mut self, c: String) -> Self {
        self.exec = c;

        self
    }

    /// Builds `Process` with the specified current working directory.
    ///
    /// This directory is where `exec` will be executed using the runtime `BinCommand`.
    pub fn cwd(mut self, d: String) -> Self {
        self.cwd = d;

        self
    }

    /// Builds `Process` with the specified log monitors.
    pub fn log_monitors(mut self, m: Vec<String>) -> Self {
        self.log_monitors = m;

        self
    }

    /// Builds `Process` with the name of the action to execute if the `exec` fails.
    pub fn onfail(mut self, f: Option<String>) -> Self {
        self.onfail = f;

        self
    }

    /// Builds `Process` with the name of the action to execute if the `exec` succeeds.
    pub fn onsucceed(mut self, s: Option<String>) -> Self {
        self.onsucceed = s;

        self
    }

    /// Executes the process using the provided actions, context, and log monitor connections.
    pub fn run(
        &self,
        actions: ProcessActions,
        ctx: &Ctx,
        log_monitor_senders: &[Sender<LogMonitorMessage>],
    ) -> Result<()> {
        debug!("Initiating process \"{}\"", self.name);

        let BinCommand { bin, mut args } = ctx.bin_command.clone();

        debug!(
            "Building exec and invoking on local binary \"{}\" with args {:?}",
            bin, args
        );

        let mut child;

        #[cfg(windows)]
        {
            child = Command::new(bin)
                .args(args)
                .raw_arg(self.exec.clone())
                .current_dir(&self.cwd[..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context(format!(
                    "Error spawning process exec on process \"{}\"",
                    self.name
                ))?;
        }

        #[cfg(not(windows))]
        {
            args.push(self.exec.clone());

            child = Command::new(bin)
                .args(args)
                .current_dir(&self.cwd[..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context(format!(
                    "Error spawning process exec on process \"{}\"",
                    self.name
                ))?;
        }

        info!("\"{}\" ({}) spawned", self.name, child.id());

        let pid = child.id();

        debug!("Begin streaming output from \"{}\" ({})", self.name, pid);
        PipeStreamReader::stream_child_output(&mut child, log_monitor_senders)
            .context("Output stream error")?;

        debug!("Waiting on close... \"{}\" ({})", self.name, pid);
        let status = child.wait().context(format!(
            "Error waiting for process exec child on process \"{}\"",
            self.name
        ))?;

        debug!(
            "Process \"{}\" ({}) closed with exit status: {:?}",
            self.name, pid, status
        );

        for sender in log_monitor_senders.iter() {
            if sender
                .send(LogMonitorMessage::new().cmd(LogMonitorCmd::Close))
                .is_err()
            {
                bail!(
                    "Error sending process close message to log monitor on process \"{}\"",
                    self.name
                );
            }
        }

        if status.success() {
            info!("\"{}\" ({}) succeeded", self.name, pid);

            if let Some(onsucceed) = actions.onsucceed {
                let onsucceed_name = match &self.onsucceed {
                    Some(n) => n.clone(),
                    None => "".to_string(),
                };
                debug!(
                    "Running onsucceed \"{}\" from prepared actions",
                    onsucceed_name
                );

                onsucceed();
            }
        } else {
            info!("\"{}\" ({}) failed", self.name, pid);

            if let Some(onfail) = actions.onfail {
                let onfail_name = match &self.onfail {
                    Some(n) => n.clone(),
                    None => "".to_owned(),
                };
                debug!("Running onfail \"{}\" from prepared actions", onfail_name);

                onfail();
            }
        }

        Ok(())
    }
}
