#[derive(Clone)]
pub enum LogMonitorCmd {
    Close,
    Log,
    None,
}

#[derive(Clone)]
pub struct LogMonitorMessage {
    pub cmd: LogMonitorCmd,
    pub message: String,
}

impl Default for LogMonitorMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl LogMonitorMessage {
    pub fn new() -> Self {
        Self {
            cmd: LogMonitorCmd::None,
            message: "Empty message.".to_owned(),
        }
    }

    pub fn cmd(mut self, c: LogMonitorCmd) -> Self {
        self.cmd = c;

        self
    }

    pub fn message(mut self, m: String) -> Self {
        self.message = m;

        self
    }
}
