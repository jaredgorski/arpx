use crate::commands::run::actions::act;
use crate::commands::run::handlers::monitor;
use crate::commands::run::processes::stream_read::{PipeStreamReader, PipedLine};
use crate::commands::run::processes::Process;
use crate::profile::Profile;
use crate::util::log;
use crossbeam_channel::Select;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn handle_output(profile: &Profile, proc: &Arc<Mutex<Process>>) {
    let mut channels: Vec<PipeStreamReader> = Vec::new();
    channels.push(PipeStreamReader::new(Box::new(
        proc.lock().unwrap().child.stdout.take().expect("!stdout"),
    )));
    channels.push(PipeStreamReader::new(Box::new(
        proc.lock().unwrap().child.stderr.take().expect("!stderr"),
    )));

    let mut select = Select::new();
    for channel in channels.iter() {
        select.recv(&channel.lines);
    }

    let mut stream_eof = false;

    while !stream_eof {
        let operation = select.select();
        let index = operation.index();
        let received = operation.recv(&channels.get(index).expect("!channel").lines);

        match received {
            Ok(remote_result) => match remote_result {
                Ok(piped_line) => match piped_line {
                    PipedLine::Line(line) => {
                        handle_output_line(&profile, &proc, line);
                    }
                    PipedLine::EOF => {
                        stream_eof = true;
                        select.remove(index);
                    }
                },
                Err(error) => log::error(&format!("{:?}", error)),
            },
            Err(_) => {
                stream_eof = true;
                select.remove(index);
            }
        }
    }

    let status = proc.lock().unwrap().child.wait().expect("!wait");
    let proc_name = proc.lock().unwrap().name[..].to_string();
    if status.success() {
        let onsucceed = proc.lock().unwrap().onsucceed[..].to_string();

        if onsucceed.is_empty() {
            let annotated_message = format!("[{}] exited with success.", proc_name);
            log::logger(&annotated_message);
        } else {
            let annotated_message = format!("[{}] onsucceed: {}", proc_name, &onsucceed);
            log::logger(&annotated_message);

            let tmp_log_data = log::LogData {
                message: "Triggering onsucceed.",
                snippets: HashMap::new(),
            };

            act(profile, proc, &tmp_log_data, &onsucceed, true);
        }
    } else {
        let onfail = proc.lock().unwrap().onfail[..].to_string();

        if onfail.is_empty() {
            let annotated_message = format!("[{}] exited with failure.", proc_name);
            log::logger(&annotated_message);
        } else {
            let annotated_message = format!("[{}] onfail: {}", proc_name, onfail);
            log::logger(&annotated_message);

            let tmp_log_data = log::LogData {
                message: "Triggering onfail.",
                snippets: HashMap::new(),
            };

            act(profile, proc, &tmp_log_data, &onfail, true);
        }
    }
}

pub fn handle_output_line(profile: &Profile, proc: &Arc<Mutex<Process>>, line: String) {
    let mut log_data = log::LogData {
        message: &line,
        snippets: HashMap::new(),
    };

    monitor::handle_monitor(profile, &proc, &mut log_data);
}
