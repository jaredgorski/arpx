pub mod task;

use crate::runtime::ctx::Ctx;
use log::debug;
use task::Task;

#[derive(Clone, Debug)]
pub struct Job {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Job {
    pub fn new(name: String, tasks: Vec<Task>) -> Self {
        Self { name, tasks }
    }

    pub fn run(self, ctx: &Ctx) -> Result<(), std::io::Error> {
        debug!(
            "Running job instance \"{}\" with structure:\n{:#?}",
            self.name, self
        );

        self.tasks
            .iter()
            .try_for_each(|task| task.clone().run(&ctx.clone()))
    }
}
