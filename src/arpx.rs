use std::collections::HashMap;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::thread::{Builder, JoinHandle};

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::action::act;
use crate::process::uplink_message::UplinkMessage;
use crate::process::Process;
use crate::profile::{ActionCfg, MonitorCfg, ProcessCfg, Profile};

#[derive(Debug, Clone)]
pub struct Arpx {
    pub actions: Vec<ActionCfg>,
    pub monitors: Vec<MonitorCfg>,
    pub processes: Vec<String>,
    pub profile: Profile,
    pub profile_processes_map: HashMap<String, ProcessCfg>,
    pub running_processes_map: Arc<Mutex<HashMap<String, Arc<Mutex<Process>>>>>,
    pub uplink: Sender<UplinkMessage>,
    pub uplink_receiver: Receiver<UplinkMessage>,
}

impl Default for Arpx {
    fn default() -> Self {
        Self::new()
    }
}

impl Arpx {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<UplinkMessage>();

        Self {
            actions: Vec::new(),
            monitors: Vec::new(),
            processes: Vec::new(),
            profile: Profile::new(),
            profile_processes_map: HashMap::new(),
            running_processes_map: Arc::new(Mutex::new(HashMap::new())),
            uplink: sender,
            uplink_receiver: receiver,
        }
    }

    pub fn load_profile(mut self, filepath: String) -> Arpx {
        self.profile = match Profile::from_file(filepath) {
            Ok(profile) => profile,
            Err(error) => panic!(error),
        };

        for profile_process in &self.profile.processes {
            self.profile_processes_map.insert(
                profile_process.name[..].to_string(),
                profile_process.clone(),
            );
        }

        let profile_monitors = self.profile.monitors.clone();
        for profile_monitor in profile_monitors {
            self.monitors.push(profile_monitor);
        }

        let profile_actions = self.profile.actions.clone();
        for profile_action in profile_actions {
            self.actions.push(profile_action);
        }

        self
    }

    pub fn run(mut self, processes: Vec<String>) -> Result<(), Error> {
        if processes.is_empty() {
            for process in self.profile.processes.iter() {
                self.processes.push(process.name[..].to_string());
            }
        }

        if self.processes.is_empty() {
            panic!("!processes");
        }

        let uplink_receiver_clone = self.uplink_receiver.clone();
        Builder::new()
            .spawn({
                let mut this = self.clone();
                move || loop {
                    let received = uplink_receiver_clone.recv();

                    match received {
                        Ok(uplink_message) => match &uplink_message.cmd[..] {
                            "execute_action" => {
                                this.act(uplink_message);
                            }
                            "remove_running_process" => {
                                println!("COMMAND TO REMOVE...");
                                this.remove_running_process(uplink_message.pid);
                            }
                            _ => (),
                        },
                        Err(_) => (),
                    }
                }
            })
            .expect("could not spawn listener thread");

        let mut process_handles = Vec::new();
        for process_name in &self.processes.clone() {
            let (handle, process) = self.run_process(process_name[..].to_string());

            if process.lock().unwrap().blocking {
                handle.join().expect("!join");
                process.lock().unwrap().wait();
            } else {
                process_handles.push(handle);
            }
        }

        // TODO: Consider implementing a Join On Drop here
        // --> https://matklad.github.io/2019/08/23/join-your-threads.html

        for handle in process_handles {
            handle.join().expect("!join");
        }

        // TODO: Add graceful shutdown check of running processes hashmap and `wait` on any
        // remaining processes (with a timeout, or kill after timeout)
        while self.running_processes_map.lock().unwrap().len() > 0 {}

        Ok(())
    }

    pub fn run_process(&mut self, process_name: String) -> (JoinHandle<()>, Arc<Mutex<Process>>) {
        println!("RUNNING PROCESS: {}", process_name);
        let process_cfg = self
            .profile_processes_map
            .get(&process_name)
            .expect("Internal process does not match any profile process.")
            .clone();

        self.run_process_from_cfg(&process_cfg)
    }

    pub fn run_process_from_cfg(
        &mut self,
        process_cfg: &ProcessCfg,
    ) -> (JoinHandle<()>, Arc<Mutex<Process>>) {
        let actions_clone = self.actions.clone();
        let monitors_clone = self.monitors.clone();
        let uplink_clone = self.uplink.clone();
        let process = Arc::new(Mutex::new(Process::new(
            actions_clone,
            monitors_clone,
            process_cfg,
            uplink_clone,
        )));
        let process_clone = Arc::clone(&process);
        let process_clone_2 = Arc::clone(&process);

        let pid = process.lock().unwrap().pid.clone();
        self.add_running_process(pid, process);

        (
            Builder::new()
                .name(process_cfg.name[..].to_string())
                .spawn(move || {
                    process_clone.lock().unwrap().run();
                })
                .expect("Could not spawn process thread"),
            process_clone_2,
        )
    }

    fn add_running_process(&self, pid: String, process: Arc<Mutex<Process>>) {
        self.running_processes_map
            .lock()
            .unwrap()
            .insert(pid, process);
    }

    fn remove_running_process(&self, pid: String) {
        self.running_processes_map.lock().unwrap().remove(&pid);
        println!("REMOVED PROCESS: {}", pid);
    }

    fn act(&mut self, uplink_message: UplinkMessage) {
        let UplinkMessage {
            action,
            pid,
            process_name,
            ..
        } = uplink_message;
        let process = self
            .running_processes_map
            .lock()
            .unwrap()
            .get(&pid)
            .unwrap()
            .clone();

        act(self, action, pid, process, process_name);
    }
}
