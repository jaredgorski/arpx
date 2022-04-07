use crate::runtime::profile::deserialize::defaults;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Process {
    #[serde(default = "defaults::string")]
    pub exec: String,
    #[serde(default = "defaults::string_vec")]
    pub log_monitors: Vec<String>,
    #[serde(default = "defaults::string")]
    pub name: String,
    #[serde(default = "defaults::cwd")]
    pub cwd: String,
    #[serde(default = "defaults::string")]
    pub onsucceed: String,
    #[serde(default = "defaults::string")]
    pub onfail: String,
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Process>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(Process);

    let deserialized = HashMap::<String, Wrapper>::deserialize(deserializer)?;
    Ok(deserialized
        .into_iter()
        .map(|(k, Wrapper(mut v))| {
            v.name = k.clone();

            (k, v)
        })
        .collect())
}
