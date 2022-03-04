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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Process {
    pub command: String,
    pub cwd: String,
    pub log_monitors: Vec<String>,
    pub name: String,
    pub onfail: Option<String>,
    pub onsucceed: Option<String>,
}

impl Process {
    pub fn new(name: String) -> Self {
        Self {
            command: String::new(),
            cwd: ".".to_owned(),
            log_monitors: Vec::new(),
            name,
            onfail: None,
            onsucceed: None,
        }
    }

    pub fn command(mut self, c: String) -> Self {
        self.command = c;

        self
    }

    pub fn cwd(mut self, d: String) -> Self {
        self.cwd = d;

        self
    }

    pub fn log_monitors(mut self, m: Vec<String>) -> Self {
        self.log_monitors = m;

        self
    }

    pub fn onfail(mut self, f: Option<String>) -> Self {
        self.onfail = f;

        self
    }

    pub fn onsucceed(mut self, s: Option<String>) -> Self {
        self.onsucceed = s;

        self
    }

    pub fn run(
        &self,
        actions: ProcessActions,
        ctx: &Ctx,
        log_monitor_senders: &[Sender<LogMonitorMessage>],
    ) -> Result<()> {
        debug!("Initiating process \"{}\"", self.name);

        let BinCommand { bin, mut args } = ctx.bin_command.clone();
        args.push(self.command.clone());

        debug!(
            "Building command and invoking on local binary \"{}\" with args {:?}",
            bin, args
        );
        let mut child = Command::new(bin)
            .args(args)
            .current_dir(&self.cwd[..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(format!(
                "Error spawning process command on process \"{}\"",
                self.name
            ))?;

        info!("\"{}\" ({}) spawned", self.name, child.id());

        let pid = child.id();

        debug!("Begin streaming output from \"{}\" ({})", self.name, pid);
        PipeStreamReader::stream_child_output(&mut child, log_monitor_senders)
            .context("Output stream error")?;

        debug!("Waiting on close... \"{}\" ({})", self.name, pid);
        let status = child.wait().context(format!(
            "Error waiting for process command child on process \"{}\"",
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
