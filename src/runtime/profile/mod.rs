mod deserialize;
mod runtime;

use crate::runtime::Runtime;
pub use deserialize::Profile;
use log::debug;
use runtime::runtime_from_profile;
use std::fs;

impl Profile {
    pub fn load_runtime(path: &str, job_names: Vec<String>) -> Result<Runtime, std::io::Error> {
        debug!("Loading profile from path: {}", path);

        let data = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(error) => return Err(error),
        };

        let profile = match Self::deserialize_from_str(&data) {
            Ok(p) => p,
            Err(error) => panic!("{}", error),
        };

        runtime_from_profile(profile, &job_names)
    }

    fn deserialize_from_str(data: &str) -> Result<Self, std::io::Error> {
        debug!("Deserializing profile data");

        match serde_yaml::from_str(data) {
            Ok(deserialized) => Ok(deserialized),
            Err(error) => panic!("{:?}", error),
        }
    }
}
