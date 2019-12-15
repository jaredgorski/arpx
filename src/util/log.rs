use std::collections::HashMap;

#[derive(Debug)]
pub struct LogData<'a> {
    pub message: &'a str,
    pub snippets: HashMap<String, String>,
}

pub fn logger(log: &str) {
    let log_output: String = log.to_string();
    println!("{}", log_output);
}

pub fn error(err: &str) {
    let error: String = format!("error: {}", err);
    println!("{}", error);
}

pub fn log_trigger_snippet(log_data: &LogData, action: &str) {
    if let Some(snippet) = log_data.snippets.get(action) {
        logger(&format!(
            "[arpx] Condition met on line:\n[arpx]\t--> {}",
            snippet
        ));
    }
}
