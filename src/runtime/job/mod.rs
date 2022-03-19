pub mod task;

use crate::runtime::ctx::Ctx;
use anyhow::Result;
use log::debug;
use task::Task;

/// Represents and contains a given runtime job.
///
/// This object contains the job name as well as the tasks the job performs. When run, all defined
/// tasks are executed in order.
#[derive(Clone, Debug)]
pub struct Job {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Job {
    /// Constructs a new, empty `Job`.
    pub fn new(name: String, tasks: Vec<Task>) -> Self {
        Self { name, tasks }
    }

    /// Executes defined tasks in order.
    pub fn run(self, ctx: &Ctx) -> Result<()> {
        debug!(
            "Running job instance \"{}\" with structure:\n{:#?}",
            self.name, self
        );

        self.tasks
            .iter()
            .try_for_each(|task| task.clone().run(&ctx.clone()))
    }
}
