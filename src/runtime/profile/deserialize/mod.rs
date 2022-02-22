mod defaults;
mod jobs;
mod processes;

use arpx_job_parser::Job;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Profile {
    #[serde(default = "defaults::jobs", deserialize_with = "jobs::deserialize")]
    pub jobs: HashMap<String, Job>,
    #[serde(
        default = "defaults::processes",
        deserialize_with = "processes::deserialize"
    )]
    pub processes: HashMap<String, processes::Process>,
}
