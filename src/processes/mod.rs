use std::process::{Command, Child, Stdio};

pub mod stream_read;

#[derive(Debug)]
pub struct Process {
    pub child: Child,
    pub name: String,
    pub pid: String,
}

impl Process {
    pub fn init(name: String, cwd: &str, command: &str) -> Process {
        let child: Child = Command::new("sh")
            .args(&["-c", command])
            .current_dir(cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("!spawn");

        Process {
            child: child,
            name: name,
            pid: "blah".to_string(),
        }
    }
}
