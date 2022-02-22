mod stream;

use crate::runtime::{
    ctx::Ctx,
    job::task::{
        action::BUILTIN_ACTIONS,
        uplink::{Cmd, UplinkMessage},
    },
};
use crossbeam_channel::Sender;
use log::{debug, error, info};
use std::process::{Command, Stdio};
use stream::PipeStreamReader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Process {
    pub command: String,
    pub cwd: String,
    pub name: String,
    pub onfail: Option<String>,
    pub onsucceed: Option<String>,
    pub silent: bool,
}

impl Process {
    pub fn new(process: Self) -> Self {
        process
    }

    pub fn run(self, ctx: Ctx, uplink: Sender<UplinkMessage>) {
        debug!("Initiating process \"{}\"", self.name);

        let bin = ctx.bin_command.bin.clone();
        let mut bin_args = ctx.bin_command.args.clone();
        bin_args.push(self.command.clone());

        debug!(
            "Building command and invoking on local binary \"{}\" with args {:?}",
            bin, bin_args
        );
        let mut child = Command::new(bin)
            .args(bin_args)
            .current_dir(&self.cwd[..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("!spawn");

        info!("\"{}\" ({}) spawned", self.name, child.id());

        let pid = child.id();

        debug!("Begin streaming output from \"{}\" ({})", self.name, pid);
        PipeStreamReader::stream_child_output(&mut child, self.silent);

        debug!("Waiting on close... \"{}\" ({})", self.name, pid);
        let status = child.wait().expect("!wait");

        debug!(
            "Process \"{}\" ({}) closed with exit status: {:?}",
            self.name, pid, status
        );

        if status.success() {
            info!("\"{}\" ({}) succeeded", self.name, pid);

            if let Some(onsucceed) = self.onsucceed {
                debug!("Indexing onsucceed \"{}\" from builtin actions", onsucceed);
                if BUILTIN_ACTIONS.contains(&&onsucceed[..]) {
                    uplink
                        .send(UplinkMessage::new().cmd(Cmd::Action).message(onsucceed))
                        .expect("!uplink send");
                } else {
                    debug!("Indexing onsucceed \"{}\" from process library", onsucceed);
                    match ctx.process_lib.get(&onsucceed[..]) {
                        Some(process) => {
                            let mut cloned_process = process.clone();

                            cloned_process.silent = self.silent;
                            cloned_process.run(ctx.clone(), uplink);
                        }
                        None => {
                            error!("No onsucceed \"{}\" found for current runtime.", onsucceed)
                        }
                    };
                }
            }
        } else {
            info!("\"{}\" ({}) failed", self.name, pid);

            if let Some(onfail) = self.onfail {
                debug!("Indexing onfail \"{}\" from builtin actions", onfail);
                if BUILTIN_ACTIONS.contains(&&onfail[..]) {
                    uplink
                        .send(UplinkMessage::new().cmd(Cmd::Action).message(onfail))
                        .expect("!uplink send");
                } else {
                    debug!("Indexing onfail \"{}\" from process library", onfail);
                    match ctx.process_lib.get(&onfail[..]) {
                        Some(process) => {
                            let mut cloned_process = process.clone();

                            cloned_process.silent = self.silent;
                            cloned_process.run(ctx.clone(), uplink);
                        }
                        None => {
                            error!("No onfail \"{}\" found for current runtime.", onfail)
                        }
                    };
                }
            }
        }
    }
}
