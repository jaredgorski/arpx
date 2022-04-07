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

    #[must_use]
    pub fn from_preset(preset: &str) -> Self {
        match preset {
            // "cmd" => Self::new("cmd".into(), vec!["/c".into()]), PENDING https://github.com/rust-lang/rust/issues/92939
            "bash" => Self::new("sh".into(), vec!["-c".into()]),
            "powershell" => Self::new("powershell".into(), vec!["-Command".into()]),
            _ => Self::new("sh".into(), vec!["-c".into()]),
        }
    }

    fn from_os(os: &str) -> Self {
        match os {
            "windows" => Self::from_preset("powershell"),
            _ => Self::from_preset("bash"),
        }
    }
}
