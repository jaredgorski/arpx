use crossbeam_channel::{Select};
use crate::processes::Process;
use crate::processes::stream_read::{PipedLine, PipeStreamReader};
use crate::util;

pub fn handle_output(mut proc: Process) {
    let mut channels: Vec<PipeStreamReader> = Vec::new();
    channels.push(PipeStreamReader::new(Box::new(proc.child.stdout.take().expect("!stdout"))));
    channels.push(PipeStreamReader::new(Box::new(proc.child.stderr.take().expect("!stderr"))));

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
            Ok(remote_result) => {
                match remote_result {
                    Ok(piped_line) => {
                        match piped_line {
                            PipedLine::Line(line) => {
                                handle_output_line(line);
                            },
                            PipedLine::EOF => {
                                stream_eof = true;
                                select.remove(index);
                            }
                        }
                    }
                    Err(error) => util::log::error(&format!("{:?}", error)),
                }
            }
            Err(_) => {
                stream_eof = true;
                select.remove(index);
            }
        }
    }

    // TODO: handle error output on child exit
    let status = proc.child.wait().expect("!wait");
    if !status.success() {
        panic!("!status: {}", status.code().expect("!code"))
    }
}

pub fn handle_output_line(line: String) {
    println!("TOM: {}", line);
}
