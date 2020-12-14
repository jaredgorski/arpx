use std::io::{self, Write};
use std::str::FromStr;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct AnnotatedMessage;

impl AnnotatedMessage {
    pub fn new<'a>(annotation: &'a str, message: &'a str) -> String {
        format!("[{}] {}", annotation, message)
    }
}

pub fn logger(log: String) {
    write_with_color(log, Color::White).expect("!log");
}

pub fn logger_with_color(log: String, color: String) {
    let log_color = {
        if color.is_empty() {
            Color::White
        } else {
            Color::from_str(&color[..]).unwrap()
        }
    };

    write_with_color(log, log_color).expect("!log");
}

pub fn logger_error(err: String) {
    write_error(err).expect("!log");
}

fn write_with_color(output: String, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", output)
}

fn write_error(output: String) -> io::Result<()> {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln!(&mut stderr, "{}", output)
}
