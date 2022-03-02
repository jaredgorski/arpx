use arpx_job_parser::{parse_job, Job};
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Job>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "job_from_str")] Job);

    let v = HashMap::<String, Wrapper>::deserialize(deserializer)?;
    Ok(v.into_iter().map(|(k, Wrapper(v))| (k, v)).collect())
}

fn job_from_str<'de, D>(deserializer: D) -> Result<Job, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|job_str| {
        parse_job(&job_str[..]).map_err(|((line, col), msg)| {
            de::Error::custom(format!(
                "[Parse error at job line {} column {}: `{}`]",
                line, col, msg
            ))
        })
    })?
}
