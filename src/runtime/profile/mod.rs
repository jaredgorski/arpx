mod deserialize;
mod runtime_builder;

use crate::runtime::Runtime;
use anyhow::{Context, Error, Result};
pub use deserialize::Profile;
use log::debug;
use runtime_builder::RuntimeBuilder;
use std::fs;

/// Represents and contains a runtime object defined by a profile.
///
/// This object can create a `Runtime` object from a file which defines that object using an
/// agreed-upon specification, known as a "profile". A profile contains `jobs`, `processes` and,
/// optionally, `log_monitors`. Items in `jobs` use a simple, domain-specific script language
/// called "arpx-job" in order to construct runtime objects using the defined `processes` and
/// `log_monitors`.
///
/// For example:
///
/// ```text
/// jobs:
///     - job1: |
///         p1 ? p2 : p3; @m1 @m2
///         p4
///         p5
///
/// processes:
///     - p1:
///         command: |
///             echo foo
///             exit 1
///     - p2:
///         command: echo bar
///     - p3:
///         command: echo baz
///     - p4:
///         command: echo qux
///     - p5:
///         command: echo quux
///     - p6:
///         command: echo garply
///
/// log_monitors:
///     - m1:
///         ontrigger: p6
///         test: 'grep "baz" <<< "$ARPX_BUFFER"'
///     - m2:
///         buffer_size: 1
///         test: 'echo $ARPX_BUFFER >> ~/test.log'
///
/// // `job1` output:
/// //
/// // [p1] "p1" (1) spawned
/// // [p1] foo
/// // [p1] "p1" (1) failed
/// // [p1] "p3" (2) spawned
/// // [p1] baz
/// // [p1] "p3" (2) succeeded
/// // [p1] "p6" (3) spawned
/// // [p1] garply
/// // [p1] "p6" (3) succeeded
/// // [p4] "p4" (4) spawned
/// // [p4] qux
/// // [p4] "p4" (4) succeeded
/// // [p5] "p5" (5) spawned
/// // [p5] quux
/// // [p5] "p5" (5) succeeded
/// ```
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
