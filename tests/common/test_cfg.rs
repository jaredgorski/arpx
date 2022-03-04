use anyhow::{Context, Result};
use arpx::BinCommand;
use std::{
    collections::HashMap,
    env::temp_dir,
    fs::{write, File},
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

const BIN_PATH: &str = "target/debug/arpx";

pub struct TC {
    bin_path: String,
    datetime: String,
    profile_path: String,
    pub envs: HashMap<String, String>,
    pub name: String,
    pub opts: String,
    pub profile: String,
}

impl TC {
    pub fn new(name: &str) -> Self {
        Self {
            bin_path: BIN_PATH.to_owned(),
            datetime: Self::datetime(),
            envs: HashMap::new(),
            name: name.to_owned(),
            opts: String::new(),
            profile: String::new(),
            profile_path: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.envs.insert(key.to_string(), value.to_string());

        self
    }

    pub fn profile(mut self, input: &str) -> Self {
        self.profile.push_str(input);

        let mut dir = temp_dir();
        dir.push(format!("{}_{}.yaml", self.name, self.datetime));
        self.profile_path = dir.as_path().display().to_string();

        File::create(dir)
            .context("Test profile creation failed")
            .unwrap();
        write(&self.profile_path, &self.profile).unwrap();

        self
    }

    pub fn opts(mut self, input: &str) -> Self {
        self.opts.push_str(input);

        self
    }

    pub fn run(self) -> Result<(Vec<String>, Vec<String>)> {
        let BinCommand { bin, mut args } = BinCommand::system_default();

        let test_command = format!("{} -f {} {}", self.bin_path, self.profile_path, self.opts);
        args.push(test_command);

        let output = Command::new(bin)
            .args(args)
            .envs(self.envs)
            .current_dir(String::from(env!("CARGO_MANIFEST_DIR")))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn test command")?
            .wait_with_output()
            .context("Failed to wait test command")?;

        let out = BufReader::new(&*output.stdout)
            .lines()
            .map(|l| l.expect("!parse"))
            .collect();
        let err = BufReader::new(&*output.stderr)
            .lines()
            .map(|l| l.expect("!parse"))
            .collect();

        Ok((out, err))
    }

    fn datetime() -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string()
    }
}
