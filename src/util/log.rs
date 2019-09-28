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

pub fn usage(bin: &str, arg_doc: Option<&Vec<String>>) {
    let mut usage: String = "\nUSAGE:".to_string();

    let doc_from_str = |bin: &str, doc: &str| -> String {
        return format!("\n    {} {}", bin, doc);
    };

    if let Some(doc) = arg_doc {
        for item in doc.iter().skip(1) {
            usage += doc_from_str(bin, &item).as_str();
        }
    } else {
        let default_doc: &str = "[-p | -r | MODE] [OPTIONS]";
        usage += doc_from_str(bin, default_doc).as_str();
    }

    println!("{}\n", usage);
}

pub fn log_trigger_snippet(log_data: &LogData, action: &str) {
    if let Some(snippet) = log_data.snippets.get(action) {
        logger(&format!("Caught trigger: {}", snippet));
    }
}
