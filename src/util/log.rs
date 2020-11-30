pub fn logger(log: &str) {
    let log_output: String = log.to_string();
    println!("{}", log_output);
}

pub fn error(err: &str) {
    let error: String = format!("error: {}", err);
    println!("{}", error);
}
