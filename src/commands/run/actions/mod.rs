use crate::profile::Profile;
use crate::commands::run::processes::Process;
use crate::util::log::LogData;

pub mod builtin;
pub mod custom;

pub fn act(profile: &Profile, proc: &mut Process, log_data: &LogData, action: &str) {
    if builtin::BUILTINS.contains(&action) {
        builtin::act(profile, proc, log_data, &action[..]);
    } else {
        custom::act(profile, proc, log_data, &action[..]);
    }
}
