use std::process::{Child, Command, Stdio};

pub mod stream_read;

#[derive(Debug)]
pub struct Process {
    pub child: Child,
    pub name: String,
    pub silent: bool,
    pub blocking: bool,
}

impl Process {
    pub fn init(name: String, cwd: &str, command: &str, silent: &bool, blocking: &bool) -> Process {
        let child: Child = Command::new("sh")
            .args(&["-c", command])
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("!spawn");

        Process {
            child: child,
            name: name,
            silent: *silent,
            blocking: *blocking,
        }
    }
}
