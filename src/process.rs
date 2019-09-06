// use std::io::{Stdout};
use std::process::{Command, Child};

#[derive(Debug)]
pub struct Process {
    pub child: Child,
    // pub logx: Stdout,
    pub name: String,
    pub pid: String,
}

impl Process {
    pub fn spawn(command: &str) {
        let child: Child = Command::new("sh")
            .args(&["-c", command])
            .spawn()
            .expect("failed to execute command");
    }
}
