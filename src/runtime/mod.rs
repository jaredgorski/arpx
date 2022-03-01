pub mod ctx;
mod job;
pub mod local_bin;
mod profile;

use crate::runtime::job::task::{log_monitor::LogMonitor, process::Process};
use ctx::Ctx;
use job::Job;
use local_bin::BinCommand;
use log::debug;
use profile::Profile;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Runtime {
    pub ctx: Ctx,
    pub jobs: Vec<Job>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            ctx: Ctx::new(),
            jobs: Vec::new(),
        }
    }

    pub fn jobs(mut self, j: Vec<Job>) -> Self {
        self.jobs = j;

        self
    }

    pub fn log_monitor_lib(mut self, p: HashMap<String, LogMonitor>) -> Self {
        self.ctx.log_monitor_lib = p;

        self
    }

    pub fn process_lib(mut self, p: HashMap<String, Process>) -> Self {
        self.ctx.process_lib = p;

        self
    }

    pub fn bin_command(&mut self, c: BinCommand) -> &Self {
        self.ctx.bin_command = c;

        self
    }

    pub fn from_profile(path: &str, job_names: Vec<String>) -> Result<Self, std::io::Error> {
        debug!("Loading runtime from profile");

        Profile::load_runtime(path, job_names)
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        debug!("Running runtime instance with structure:\n{:#?}", self);

        self.jobs
            .iter()
            .try_for_each(|job| job.clone().run(self.ctx.clone()))
    }
}
