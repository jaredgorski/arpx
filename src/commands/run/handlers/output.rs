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
    if status.success() {
        let annotated_message = &format!("[{}] exited with success.", proc.lock().unwrap().name);
        log::logger(annotated_message);
    } else {
        let annotated_message = &format!("[{}] exited with failure.", proc.lock().unwrap().name);
        log::logger(annotated_message);
    }
}

pub fn handle_output_line(profile: &Profile, proc: &Arc<Mutex<Process>>, line: String) {
    let mut log_data = log::LogData {
        message: &line,
        snippets: HashMap::new(),
    };

    monitor::handle_monitor(profile, &proc, &mut log_data);
}
