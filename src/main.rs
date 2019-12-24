extern crate arpx;

use arpx::commands::{get_command, run, Command};
use clap::{App, Arg};

pub const APPNAME: &str = "arpx";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
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
                .help("Path to the profile to be executed")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("process")
                .short("p")
                .long("process")
                .value_name("PROCESS")
                .help("Specifies a process in the profile to run individually")
                .takes_value(true),
        )
        .get_matches();

    let cmd: Command = get_command(matches);

    run::run(&cmd.profile, cmd.processes_to_run);
}
