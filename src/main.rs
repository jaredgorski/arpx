use std::io::Error;

use clap::{App, Arg};

extern crate arpx;
use arpx::arpx::Arpx;

pub const APPNAME: &str = "arpx";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> Result<(), Error> {
    let default_profile: String = format!("{}.yaml", APPNAME);
    let matches = App::new(APPNAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .default_value(&default_profile)
                .help("Path to the profile to be executed.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("process")
                .short("p")
                .long("process")
                .multiple(true)
                .value_name("PROCESS")
                .help("Specifies processes in the profile to run. Defaults to all.")
                .takes_value(true),
        )
        .get_matches();

    let requested_profile_file: String =
        matches.value_of("file").unwrap_or("arpx.yaml").to_string();

    let requested_processes: Vec<String> = {
        if matches.is_present("process") {
            matches.values_of("process").unwrap().map(|x| x.to_string()).collect()
        } else {
            Vec::new()
        }
    };

    Arpx::new()
        .load_profile(requested_profile_file)
        .run(requested_processes)
}
