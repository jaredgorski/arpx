use std::env::consts::OS;

/// Command used to execute runtime processes.
///
/// By default, this object uses Cmd when a Windows OS is detected and Bash otherwise. This object
/// contains `bin` and `args` fields which define the binary command and arguments used by the
/// runtime to execute the commands defined in processes and log monitors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinCommand {
    pub bin: String,
    pub args: Vec<String>,
}

impl BinCommand {
    #[must_use]
    pub fn new(bin: String, args: Vec<String>) -> Self {
        Self { bin, args }
    }

    #[must_use]
    pub fn system_default() -> Self {
        Self::from_os(OS)
    }

    fn from_os(os: &str) -> Self {
        let bash = Self::new("sh".into(), vec!["-c".into()]);
        let cmd = Self::new("cmd".into(), vec!["/c".into()]);

        match os {
            "windows" => cmd,
            "linux" | "macos" | "freebsd" | "netbsd" | "openbsd" | "solaris" => bash,
            _ => bash,
        }
    }
}
