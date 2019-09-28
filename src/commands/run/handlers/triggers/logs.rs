use crate::cfg::profile::LogTriggerCfg;
use crate::commands::run::handlers::monitor::MonitorOutput;
use crate::util::log::LogData;
use std::collections::HashMap;

pub fn logs_potential_pull(
    log_actions: &[String],
    log_triggers: &LogTriggerCfg,
    log_data: &LogData,
) -> MonitorOutput {
    let mut output = MonitorOutput {
        exec_actions: Vec::new(),
        snippets: HashMap::<String, String>::new(),
    };

    for action in log_actions {
        match () {
            _ if !log_triggers.includes_string.is_empty()
                && log_data
                    .message
                    .to_string()
                    .contains(&log_triggers.includes_string) =>
            {
                output.exec_actions.push(action.to_string());
                output.snippets.insert(
                    action.to_string(),
                    get_trigger_snippet(&log_data, &log_triggers.includes_string),
                );
            }
            _ => (),
        }
    }

    output
}

fn get_trigger_snippet(_log_data: &LogData, substr: &str) -> String {
    // TODO: provide context on snippets
    substr.to_string()
}
