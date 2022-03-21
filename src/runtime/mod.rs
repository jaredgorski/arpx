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
/// This object contains an ordered list of [`jobs`], each job containing an ordered list of tasks,
/// each task containing one or more concurrent processes. This object also contains a [`ctx`]
/// object which contains the binary command and args for the current runtime as well as hashmap
/// indexes for processes and log monitors which are copied to each child job, task, and process so
/// that the runtime can instantiate new processes and log monitors on the fly.
///
/// A visual representation:
///
/// ```text
///     Runtime
/// T       |
/// I       > [ Job1, Job2, ... ]
/// M             |
/// E             > [ Task1, Task2, ... ]
///                     |
/// |                   > Process1 -> `echo foo`
/// |                   |     |
/// |                   |     > OnSucceed: Process2 -> `echo bar`
/// v                   |
///                     > Process2 -> `echo bar`
///
///     ...
///
///     Runtime
///         |
///         > [ Job1, Job2, ... ]
///               |
///               > [ Task1, Task2, ... ]
///                            |
///                            > Process3 -> `echo baz`
///
///     ...
///
///     Runtime
///         |
///         > [ Job1, Job2, ... ]
///                     |
///                     > [ Task1, ... ]
///                           |
///                           > Process4 -> `echo qux`
/// ```
///
/// Processes in a given task run concurrently.
///
/// Once all processes in the task have exited, including actions (`onsucceed`, `onfail`, and log
/// monitor `ontrigger` actions spawned on their parent process threads), the task is complete and
/// the next task in the job will execute.
///
/// Once all tasks in a given job have completed their execution, the runtime moves on to the next
/// job in the queue. Once all jobs have completed their execution, the runtime is finished.
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
/// let processes = vec![
///     Process::new("p_foo".to_string())
///         .command("echo foo".to_string())
///         .onsucceed(Some("p_baz".to_string())),
///     Process::new("p_bar".to_string()).command("echo bar".to_string()),
/// ];
///
/// let mut process_map = processes
///     .clone()
///     .into_iter()
///     .map(|process| (process.name.clone(), process))
///     .collect::<HashMap<String, Process>>();
///
/// process_map.insert(
///     "p_baz".to_string(),
///     Process::new("p_baz".to_string()).command("echo baz".to_string()),
/// );
///
/// let jobs = vec![Job::new(
///     "my_job".to_string(),
///     vec![Task::new(processes)],
/// )];
///
/// Runtime::new()
///     .jobs(jobs)
///     .process_map(process_map)
///     .run();
///
/// // Output:
/// //
/// // [p_foo] "p_foo" (1) spawned
/// // [p_bar] "p_bar" (2) spawned
/// // [p_foo] foo
/// // [p_bar] bar
/// // [p_foo] "p_foo" (1) succeeded
/// // [p_bar] "p_bar" (2) succeeded
/// // [p_foo] "p_baz" (3) spawned
/// // [p_foo] baz
/// // [p_foo] "p_baz" (3) succeeded
/// ```
///
/// Note that `p_baz` runs on the `p_foo` thread, since `p_foo` executes it `onsucceed`.
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
    /// Constructs a new, empty `Runtime`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ctx: Ctx::new(),
            jobs: Vec::new(),
        }
    }

    /// Builds `Runtime` with the specified jobs.
    #[must_use]
    pub fn jobs(mut self, j: Vec<Job>) -> Self {
        self.jobs = j;

        self
    }

    /// Builds `Runtime` with the specified log monitors.
    #[must_use]
    pub fn log_monitor_map(mut self, p: HashMap<String, LogMonitor>) -> Self {
        self.ctx.log_monitor_map = p;

        self
    }

    /// Builds `Runtime` with the specified processes.
    #[must_use]
    pub fn process_map(mut self, p: HashMap<String, Process>) -> Self {
        self.ctx.process_map = p;

        self
    }

    /// Builds `Runtime` with the specified binary command.
    pub fn bin_command(mut self, c: BinCommand) -> Self {
        self.ctx.bin_command = c;

        self
    }

    /// Constructs a new `Runtime` from a profile at the specified path, using the specified jobs.
    pub fn from_profile(path: &str, job_names: &[String]) -> Result<Self> {
        debug!("Loading runtime from profile");

        Profile::load_runtime(path, job_names)
    }

    /// Executes the runtime.
    pub fn run(&self) -> Result<()> {
        debug!("Running runtime instance with structure:\n{:#?}", self);

        self.jobs
            .iter()
            .try_for_each(|job| job.clone().run(&self.ctx.clone()))
            .context("Runtime error")
    }
}
