pub mod ctx;
pub mod job;
pub mod local_bin;
pub mod profile;

use crate::runtime::job::task::{log_monitor::LogMonitor, process::Process};
use anyhow::{Context, Result};
use ctx::Ctx;
use job::Job;
use local_bin::BinCommand;
use log::debug;
use profile::Profile;
use std::collections::HashMap;

/// Represents and contains a given runtime.
///
/// This object contains an ordered list of [`jobs`], each of which contain an ordered list of
/// tasks, each containing one or more concurrent processes. This object also contains a [`ctx`]
/// object which contains the binary command and args for the current runtime as well as hashmap
/// libraries for processes and log monitors which are copied to each child job, task, and process
/// so that the runtime can instantiate new processes and log monitors on the fly.
///
/// [`jobs`]: #structfield.jobs
/// [`ctx`]: #structfield.ctx
///
/// # Examples:
///
/// Basic usage:
///
/// ```
/// use arpx::{Job, Process, Runtime, Task};
/// use std::collections::HashMap;
///
/// // Define processes
/// let processes = vec![Process::new("my_process".to_string())
///     .command("echo foo".to_string())
///     .onsucceed(Some("my_other_process".to_string()))];
///
/// // Build jobs
/// let jobs = vec![Job::new(
///     "my_job".to_string(),
///     vec![Task::new(processes.clone())],
/// )];
///
/// // Build process library
/// let mut process_lib = processes
///     .into_iter()
///     .map(|process| (process.name.clone(), process))
///     .collect::<HashMap<String, Process>>();
///
/// process_lib.insert(
///     "my_other_process".to_string(),
///     Process::new("my_other_process".to_string()).command("echo bar".to_string()),
/// );
///
/// // Instantiate runtime
/// Runtime::new()
///     .jobs(jobs)
///     .process_lib(process_lib)
///     .run()
///
/// // Output:
/// //
/// // [my_process] "my_process" (8611) spawned
/// // [my_process] foo
/// // [my_process] "my_process" (8611) succeeded
/// // [my_process] "my_other_process" (8612) spawned
/// // [my_process] bar
/// // [my_process] "my_other_process" (8612) succeeded
/// ```
#[derive(Clone, Debug)]
pub struct Runtime {
    pub ctx: Ctx,
    pub jobs: Vec<Job>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ctx: Ctx::new(),
            jobs: Vec::new(),
        }
    }

    #[must_use]
    pub fn jobs(mut self, j: Vec<Job>) -> Self {
        self.jobs = j;

        self
    }

    #[must_use]
    pub fn log_monitor_lib(mut self, p: HashMap<String, LogMonitor>) -> Self {
        self.ctx.log_monitor_lib = p;

        self
    }

    #[must_use]
    pub fn process_lib(mut self, p: HashMap<String, Process>) -> Self {
        self.ctx.process_lib = p;

        self
    }

    pub fn bin_command(&mut self, c: BinCommand) -> &Self {
        self.ctx.bin_command = c;

        self
    }

    pub fn from_profile(path: &str, job_names: &[String]) -> Result<Self> {
        debug!("Loading runtime from profile");

        Profile::load_runtime(path, job_names)
    }

    pub fn run(&self) -> Result<()> {
        debug!("Running runtime instance with structure:\n{:#?}", self);

        self.jobs
            .iter()
            .try_for_each(|job| job.clone().run(&self.ctx.clone()))
            .context("Runtime error")
    }
}
