use std::collections::HashSet;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crossbeam_channel::{Select, Sender};

use crate::error;
use crate::process::{
    stream_read::{PipeStreamReader, PipedLine},
    uplink_message::UplinkMessage,
};
use crate::profile::{ActionCfg, MonitorCfg, ProcessCfg};
use crate::util::log::{logger, logger_error, logger_with_color, AnnotatedMessage};

pub mod stream_read;
pub mod uplink_message;

/// Represents a single process run and managed by the current Arpx instance. All output from the
/// child process, as well as significant events in the process lifetime (including monitor
/// conditions being triggered and exit status) are sent via the uplink to the Arpx instance to be
/// handled on the main thread.
#[derive(Debug)]
pub struct Process {
    /// The list of custom actions from the Arpx instance, as defined in the currently-loaded
    /// profile.
    pub actions: Vec<ActionCfg>,

    /// If true, Arpx will wait for the process to exit before spawning any new processes.
    pub blocking: bool,

    /// The child process object, which executes the command.
    pub child: Child,

    /// The configured command to be executed on the process.
    pub command: String,

    /// The configured output color for the child process.
    pub color: String,

    /// The working directory in which to execute the command.
    pub cwd: String,

    /// Signals whether the process has exited.
    pub exited: bool,

    /// The list of monitors from the Arpx instance, as defined in the currently-loaded profile.
    pub monitors: Vec<MonitorCfg>,

    /// The name of the process.
    pub name: String,

    /// The name of the action to perform if the process exits with a failure code.
    pub onfail: String,

    /// The name of the action to perform if the process exits with a success code.
    pub onsucceed: String,

    /// The `id` of the child process within the operating system.
    pub pid: String,

    /// If true, process stdout will be suppressed.
    pub silent: bool,

    /// The `Sender` object received from the Arpx instance. Used for sending messages to the Arpx
    /// instance to perform actions on the main thread.
    pub uplink: Sender<UplinkMessage>,
}

/// Simple function for joining a given process and conditionally blocking it, if it's a blocking
/// process as defined in the profile.
#[doc(hidden)]
pub fn join_and_handle_blocking(
    process_with_handle: (JoinHandle<()>, Arc<Mutex<Process>>),
) -> Result<(), error::ArpxError> {
    let (handle, process) = process_with_handle;

    match handle.join() {
        Ok(()) => (),
        Err(_) => return Err(error::internal_error("".to_string())),
    };

    if process.lock().unwrap().blocking {
        process.lock().unwrap().wait_and_close();
    }

    Ok(())
}

impl Process {
    /// Builds and initiates a child process. Returns a Process instance with the proper
    /// configuration.
    pub fn init(
        actions: Vec<ActionCfg>,
        monitors: Vec<MonitorCfg>,
        process: &ProcessCfg,
        uplink: Sender<UplinkMessage>,
    ) -> Self {
        let child: Child = Command::new("sh")
            .args(&["-c", &process.command[..]])
            .current_dir(&process.cwd[..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("!spawn");

        let pid = format!("{}", child.id());

        Self {
            actions,
            blocking: process.blocking,
            child,
            command: process.command[..].to_string(),
            color: process.color[..].to_string(),
            cwd: process.cwd[..].to_string(),
            exited: false,
            monitors,
            name: process.name[..].to_string(),
            onfail: process.onfail[..].to_string(),
            onsucceed: process.onsucceed[..].to_string(),
            pid,
            silent: process.silent,
            uplink,
        }
    }

    /// Handles entire process runtime including logging child process output, monitoring the child
    /// process, communicating events to the Arpx instance, and closing the process.
    pub fn handle_runtime(&mut self) {
        self.handle_process_stream();
        self.wait_and_close();
    }

    /// SIGKILL the child process.
    pub fn kill(&mut self) {
        if !self.exited {
            self.child.kill().expect("killed");
        }
    }

    /// Wait on the child process to exit.
    pub fn wait(&mut self) -> ExitStatus {
        self.child.wait().expect("waited")
    }

    /// Wait on the child process to exit, log the process exit, and communicate the exit details
    /// to the Arpx instance.
    pub fn wait_and_close(&mut self) -> ExitStatus {
        let status = self.wait();
        let process_name = self.name[..].to_string();
        if status.success() {
            let onsucceed = self.onsucceed[..].to_string();
            if onsucceed.is_empty() {
                logger(AnnotatedMessage::new(
                    &process_name[..],
                    "exited with success",
                ));
            } else {
                logger(AnnotatedMessage::new(
                    &process_name[..],
                    &format!("onsucceed: {}", &onsucceed),
                ));
            }

            self.uplink
                .send(UplinkMessage {
                    action: onsucceed,
                    cmd: String::from("execute_action"),
                    parameters: Vec::new(),
                    pid: self.pid.clone(),
                    process_name: process_name[..].to_string(),
                })
                .expect("!send to uplink");

            self.uplink
                .send(UplinkMessage {
                    action: String::new(),
                    cmd: String::from("remove_running_process"),
                    parameters: Vec::new(),
                    pid: self.pid.clone(),
                    process_name: process_name[..].to_string(),
                })
                .expect("!send to uplink");
        } else {
            let onfail = self.onfail[..].to_string();
            if onfail.is_empty() {
                logger(AnnotatedMessage::new(
                    &process_name[..],
                    "exited with failure",
                ));
            } else {
                logger(AnnotatedMessage::new(
                    &process_name[..],
                    &format!("onfail: {}", &onfail),
                ));
            }

            self.uplink
                .send(UplinkMessage {
                    action: onfail,
                    cmd: String::from("execute_action"),
                    parameters: Vec::new(),
                    pid: self.pid.clone(),
                    process_name: process_name[..].to_string(),
                })
                .expect("!send to uplink");

            self.uplink
                .send(UplinkMessage {
                    action: String::new(),
                    cmd: String::from("remove_running_process"),
                    parameters: Vec::new(),
                    pid: self.pid.clone(),
                    process_name: process_name[..].to_string(),
                })
                .expect("!send to uplink");
        }

        self.exited = true;

        status
    }

    /// Log child process output, monitor the process, and communicate events to the Arpx instance.
    fn handle_process_stream(&mut self) {
        let mut channels: Vec<PipeStreamReader> = Vec::new();

        channels.push(PipeStreamReader::init(Box::new(
            self.child.stdout.take().expect("!stdout"),
        )));
        channels.push(PipeStreamReader::init(Box::new(
            self.child.stderr.take().expect("!stderr"),
        )));

        let mut select = Select::new();

        for channel in channels.iter() {
            select.recv(&channel.lines);
        }

        let mut stream_eof = false;

        while !stream_eof {
            let operation = select.select();
            let index = operation.index();
            let received = operation.recv(&channels.get(index).expect("!channel").lines);

            match received {
                Ok(remote_result) => match remote_result {
                    Ok(piped_line) => match piped_line {
                        PipedLine::Line(line) => {
                            let mut exec_actions: Vec<String> = Vec::new();
                            for monitor in &self.monitors {
                                if monitor.process == self.name {
                                    for action in &monitor.actions {
                                        let log_var_condition = format!(
                                            r#"
                                                LOG_LINE="{message_var}"
                                                {condition_cmd}
                                            "#,
                                            message_var = &line,
                                            condition_cmd = &monitor.condition,
                                        );

                                        let mut condition_child: Child = Command::new("sh")
                                            .args(&["-c", &log_var_condition[..]])
                                            .stdin(Stdio::piped())
                                            .stdout(Stdio::piped())
                                            .stderr(Stdio::piped())
                                            .spawn()
                                            .expect("!spawn");

                                        let status = condition_child.wait().expect("!wait");
                                        if status.success() {
                                            if !self.silent && (&action[..] != "silence") {
                                                logger(AnnotatedMessage::new(
                                                    &self.name[..],
                                                    &format!(
                                                        "Condition met. Performing: {}",
                                                        action
                                                    ),
                                                ));
                                            }

                                            exec_actions.push(action.to_string());
                                        }
                                    }
                                }
                            }

                            if !self.silent && !exec_actions.contains(&"silence".to_string()) {
                                logger_with_color(
                                    AnnotatedMessage::new(&self.name[..], &line),
                                    self.color[..].to_string(),
                                );
                            }

                            let mut action_set = HashSet::new();
                            exec_actions.retain(|x| action_set.insert(x.clone()));

                            for action in action_set {
                                self.uplink
                                    .send(UplinkMessage {
                                        action,
                                        cmd: String::from("execute_action"),
                                        parameters: Vec::new(),
                                        pid: self.pid.clone(),
                                        process_name: self.name[..].to_string(),
                                    })
                                    .expect("!send to uplink");
                            }
                        }
                        PipedLine::EOF => {
                            stream_eof = true;
                            select.remove(index);
                        }
                    },
                    Err(error) => {
                        logger_error(AnnotatedMessage::new(&self.name, &format!("{:?}", error)));
                    }
                },
                Err(_) => {
                    stream_eof = true;
                    select.remove(index);
                }
            }
        }
    }
}
