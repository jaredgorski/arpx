use std::env::consts::OS;

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
