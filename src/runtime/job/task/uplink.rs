#[derive(Clone)]
pub enum Cmd {
    Action,
    None,
}

#[derive(Clone)]
pub struct UplinkMessage {
    pub cmd: Cmd,
    pub message: String,
}

impl UplinkMessage {
    pub fn new() -> Self {
        Self {
            cmd: Cmd::None,
            message: "Empty message.".to_string(),
        }
    }

    pub fn cmd(mut self, c: Cmd) -> Self {
        self.cmd = c;

        self
    }

    pub fn message(mut self, m: String) -> Self {
        self.message = m;

        self
    }
}
