mod deserialize;
mod runtime_builder;

use crate::runtime::Runtime;
use anyhow::{Context, Error, Result};
pub use deserialize::Profile;
use log::debug;
use runtime_builder::RuntimeBuilder;
use std::fs;

impl Profile {
    pub fn load_runtime(path: &str, job_names: &[String]) -> Result<Runtime> {
        debug!("Loading profile from path: {}", path);

        let data = fs::read_to_string(path).context("Error reading file")?;
        let profile = Self::deserialize_from_str(&data).context("Error deserializing file")?;

        RuntimeBuilder::from_profile_and_job_names(profile, job_names)
            .context("Error building runtime")
    }

    fn deserialize_from_str(data: &str) -> Result<Self> {
        debug!("Deserializing profile data");

        serde_yaml::from_str(data).map_err(Error::new)
    }
}
