use std::collections::HashSet;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crossbeam_channel::{Select, Sender};

use crate::process::stream_read::{PipeStreamReader, PipedLine};
use crate::process::uplink_message::UplinkMessage;
use crate::profile::{ActionCfg, MonitorCfg, ProcessCfg};
use crate::util::log;

pub mod stream_read;
pub mod uplink_message;

#[derive(Debug)]
pub struct Process {
    pub actions: Vec<ActionCfg>,
    pub blocking: bool,
    pub child: Child,
    pub command: String,
    pub cwd: String,
    pub exited: bool,
    pub monitors: Vec<MonitorCfg>,
    pub name: String,
    pub onfail: String,
    pub onsucceed: String,
    pub pid: String,
    pub silent: bool,
    pub uplink: Sender<UplinkMessage>,
}

pub fn join_and_handle_blocking(process_with_handle: (JoinHandle<()>, Arc<Mutex<Process>>)) {
    let (handle, process) = process_with_handle;

    handle.join().expect("!join");

    if process.lock().unwrap().blocking {
        process.lock().unwrap().wait_and_close();
    }
}

impl Process {
    pub fn new(
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

    pub fn run(&mut self) {
        self.handle_process_stream();
        self.wait_and_close();
    }

    pub fn kill(&mut self) {
        if !self.exited {
            self.child.kill().expect("killed");
        }
    }

    pub fn wait(&mut self) -> ExitStatus {
        self.child.wait().expect("waited")
    }

    pub fn wait_and_close(&mut self) -> ExitStatus {
        let status = self.wait();
        let process_name = self.name[..].to_string();
        if status.success() {
            let onsucceed = self.onsucceed[..].to_string();
            if onsucceed.is_empty() {
                let annotated_message = format!("[{}] exited with success.", process_name);
                log::logger(&annotated_message);
            } else {
                let annotated_message = format!("[{}] onsucceed: {}", process_name, &onsucceed);
                log::logger(&annotated_message);
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
                let annotated_message = format!("[{}] exited with failure.", process_name);
                log::logger(&annotated_message);
            } else {
                let annotated_message = format!("[{}] onfail: {}", process_name, onfail);
                log::logger(&annotated_message);
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

    fn handle_process_stream(&mut self) {
        let mut channels: Vec<PipeStreamReader> = Vec::new();

        channels.push(PipeStreamReader::new(Box::new(
            self.child.stdout.take().expect("!stdout"),
        )));
        channels.push(PipeStreamReader::new(Box::new(
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
                                                log::logger(&format!(
                                                    "[arpx] Condition met on line:\n[arpx]\t--> {}\n[arpx]\t performing: {}",
                                                    &line,
                                                    action,
                                                ));
                                            }

                                            exec_actions.push(action.to_string());
                                        }
                                    }
                                }
                            }

                            if !self.silent && !exec_actions.contains(&"silence".to_string()) {
                                let annotated_message = &format!("[{}] {}", self.name, line);
                                log::logger(annotated_message);
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
                    Err(error) => log::error(&format!("{:?}", error)),
                },
                Err(_) => {
                    stream_eof = true;
                    select.remove(index);
                }
            }
        }
    }
}
