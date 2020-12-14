pub mod builtin;
pub mod custom;

use std::sync::{Arc, Mutex};

use crate::arpx::Arpx;
use crate::error;
use crate::process::Process;

pub fn act(
    arpx_ref: &mut Arpx,
    action: String,
    pid: String,
    process: Arc<Mutex<Process>>,
    process_name: String,
) -> Result<(), error::ArpxError> {
    if builtin::BUILTINS.contains(&&action[..]) {
        builtin::act(arpx_ref, action, pid, process, process_name)?
    } else {
        custom::act(arpx_ref, action, pid, process, process_name)?
    }

    Ok(())
}
