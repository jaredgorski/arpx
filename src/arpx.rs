use std::collections::HashMap;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::thread::{Builder, JoinHandle};

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::action::act;
use crate::process::uplink_message::UplinkMessage;
use crate::process::Process;
use crate::profile::{ActionCfg, MonitorCfg, ProcessCfg, Profile};

/// Provides an interface for loading an `arpx.yaml` profile and running one or more processes
/// defined within that profile, along with whatever monitors and subsequent actions are defined
/// for those processes.
///
/// # Example
///
/// ```no run
/// # use arpx::Arpx;
/// Arpx::new()
///     .load_profile("/path/to/arpx.yaml")
///     .run(vec![
///         String::from("list"),
///         String::from("of"),
///         String::from("processes"),
///     ])
/// ```
///
/// Each process will be run within a separate thread. If no processes are specified in `.run()`,
/// all processes defined in the loaded arpx.yaml profile will be run.
#[derive(Debug, Clone)]
pub struct Arpx {
    /// List of custom actions available to the Arpx instance, as defined in the currently-loaded
    /// profile.
    pub actions: Vec<ActionCfg>,

    /// List of monitors to be run on specific processes within the Arpx instance, as defined in
    /// the currently-loaded profile.
    pub monitors: Vec<MonitorCfg>,

    /// Map of processes available to run within the Arpx instance, as defined in the
    /// currently-loaded profile.
    pub processes: HashMap<String, ProcessCfg>,

    /// The currently-loaded `arpx.yaml` profile, deserialized.
    pub profile: Profile,

    /// List of the process names to be run at the start of the current Arpx runtime.
    pub processes_to_run: Vec<String>,

    /// A map containing currently-running process objects. This allows the Arpx instance to manage
    /// all processes within its runtime.
    pub running_processes_map: Arc<Mutex<HashMap<String, Arc<Mutex<Process>>>>>,

    /// A `Sender` object which is cloned and passed to process threads for communication with the
    /// main process of the Arpx instance. This enables child process threads to instruct the main
    /// process to spawn new processes directly from the main thread, for example.
    pub uplink: Sender<UplinkMessage>,

    /// A `Receiver` object for receiving messages from child process threads.
    pub uplink_receiver: Receiver<UplinkMessage>,
}

impl Default for Arpx {
    fn default() -> Self {
        Self::new()
    }
}

impl Arpx {
    /// Returns a new Arpx instance with all defaults.
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<UplinkMessage>();

        Self {
            actions: Vec::new(),
            monitors: Vec::new(),
            processes: HashMap::new(),
            profile: Profile::new(),
            processes_to_run: Vec::new(),
            running_processes_map: Arc::new(Mutex::new(HashMap::new())),
            uplink: sender,
            uplink_receiver: receiver,
        }
    }

    /// Loads an `arpx.yaml` profile into the Arpx instance by filepath and configures the Arpx
    /// instance according to the definitions in the profile.
    pub fn load_profile(mut self, filepath: String) -> Arpx {
        self.profile = match Profile::from_file(filepath) {
            Ok(profile) => profile,
            Err(error) => panic!(error),
        };

        for profile_process in &self.profile.processes {
            self.processes.insert(
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

    /// Runs the Arpx instance based on the current configuration, allowing for specifying one or
    /// more processes from the loaded profile to run. If no processes are specified, all processes
    /// from the loaded profile will be run.
    pub fn run(mut self, processes: Vec<String>) -> Result<(), Error> {
        if processes.is_empty() {
            for process_name in self.processes.keys() {
                self.processes_to_run.push(process_name[..].to_string());
            }
        } else {
            self.processes_to_run.extend(processes);
        }

        let uplink_receiver_clone = self.uplink_receiver.clone();
        Builder::new()
            .spawn({
                let mut this = self.clone();
                move || loop {
                    let received = uplink_receiver_clone.recv();

                    if let Ok(uplink_message) = received {
                        match &uplink_message.cmd[..] {
                            "execute_action" => {
                                this.act(uplink_message);
                            }
                            "remove_running_process" => {
                                this.remove_running_process(uplink_message.pid);
                            }
                            _ => (),
                        }
                    }
                }
            })
            .expect("could not spawn listener thread");

        let mut process_handles = Vec::new();
        for process_name in &self.processes_to_run.clone() {
            let (handle, process) = self.run_process(process_name[..].to_string());

            if process.lock().unwrap().blocking {
                handle.join().expect("!join");
                process.lock().unwrap().wait();
            } else {
                process_handles.push(handle);
            }
        }

        // TODO: Look into Join On Drop
        // --> https://matklad.github.io/2019/08/23/join-your-threads.html

        for handle in process_handles {
            handle.join().expect("!join");
        }

        while self.running_processes_map.lock().unwrap().len() > 0 {}

        Ok(())
    }

    /// Runs an individual process from the currently loaded profile by process name.
    pub fn run_process(&mut self, process_name: String) -> (JoinHandle<()>, Arc<Mutex<Process>>) {
        let process_cfg = self
            .processes
            .get(&process_name)
            .expect("Internal process does not match any profile process.")
            .clone();

        self.run_process_from_cfg(&process_cfg)
    }

    /// Runs an individual process by receiving a process configuration object. The process is run
    /// in a new thread spawned directly from the main thread.
    pub fn run_process_from_cfg(
        &mut self,
        process_cfg: &ProcessCfg,
    ) -> (JoinHandle<()>, Arc<Mutex<Process>>) {
        let actions_clone = self.actions.clone();
        let monitors_clone = self.monitors.clone();
        let uplink_clone = self.uplink.clone();
        let process = Arc::new(Mutex::new(Process::init(
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
                    process_clone.lock().unwrap().handle_runtime();
                })
                .expect("Could not spawn process thread"),
            process_clone_2,
        )
    }

    /// Internal function which adds a process object to the running_processes_map on the current
    /// Arpx instance.
    fn add_running_process(&self, pid: String, process: Arc<Mutex<Process>>) {
        self.running_processes_map
            .lock()
            .unwrap()
            .insert(pid, process);
    }

    /// Internal function which removes a process object from the running_processes_map on the
    /// current Arpx instance.
    fn remove_running_process(&self, pid: String) {
        self.running_processes_map.lock().unwrap().remove(&pid);
    }

    /// Internal function which passes an action instruction to the action handler code for
    /// execution.
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
