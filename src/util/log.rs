use std::io::{self, Write};
use std::str::FromStr;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn logger(log: &str) {
    write_with_color(log, Color::White).expect("!log");
}

pub fn logger_with_color(log: &str, color: String) {
    let log_color = {
        if color.is_empty() {
            Color::White
        } else {
            Color::from_str(&color[..]).unwrap()
        }
    };

    write_with_color(log, log_color).expect("!log");
}

pub fn error(err: &str) {
    write_error(err).expect("!log");
}

fn write_with_color(output: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", output)
}

fn write_error(output: &str) -> io::Result<()> {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln!(&mut stderr, "{}", output)
}
