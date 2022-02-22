use log::debug;

pub const BUILTIN_ACTIONS: [&str; 2] = ["arpx_exit", "arpx_exit_error"];

pub fn execute_action(action: &str) {
    match action {
        "arpx_exit" => {
            debug!("Received builtin action \"arpx_exit\". Exiting runtime.");
            std::process::exit(0)
        }
        "arpx_exit_error" => {
            debug!(
                "Received builtin action \"arpx_exit_error\". Exiting runtime with error status."
            );
            std::process::exit(1)
        }
        _ => {
            debug!("Unknown builtin action received. Doing nothing.");
        }
    }
}
