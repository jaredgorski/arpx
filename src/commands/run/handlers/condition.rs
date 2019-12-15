use crate::commands::run::handlers::monitor::MonitorOutput;
use crate::util::log::LogData;
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};

pub fn test_condition(actions: &[String], condition: &str, log_data: &LogData) -> MonitorOutput {
    let mut output = MonitorOutput {
        exec_actions: Vec::new(),
        snippets: HashMap::<String, String>::new(),
    };

    for action in actions {
        let log_var_condition = format!(
            r#"
                LOG_LINE="{message_var}"
                {condition_cmd}
            "#,
            message_var = log_data.message,
            condition_cmd = condition,
        );

        let mut condition_child: Child = Command::new("sh")
            .args(&["-c", &log_var_condition[..]])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("!spawn");

        let status = condition_child.wait().expect("!wait");
        if status.success() {
            output.exec_actions.push(action.to_string());
        }
    }

    output
}
