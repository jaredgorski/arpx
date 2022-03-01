use crate::runtime::profile::deserialize::defaults;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct LogMonitor {
    #[serde(default = "defaults::buffer_size")]
    pub buffer_size: usize,
    #[serde(default = "defaults::string")]
    pub name: String,
    #[serde(default = "defaults::string")]
    pub ontrigger: String,
    #[serde(default = "defaults::boolean")]
    pub silent: bool,
    #[serde(default = "defaults::string")]
    pub test: String,
    #[serde(default = "defaults::variable_pattern")]
    pub variable_pattern: String,
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, LogMonitor>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(LogMonitor);

    let v = HashMap::<String, Wrapper>::deserialize(deserializer)?;
    Ok(v.into_iter()
        .map(|(k, Wrapper(mut v))| {
            v.name = k.clone();

            (k, v)
        })
        .collect())
}
