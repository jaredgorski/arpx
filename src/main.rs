extern crate tog;

use clap::{Arg, App, SubCommand};
use tog::commands::{Command, get_command, run};

const TOG_VERSION: &str = env!("CARGO_PKG_VERSION");
const TOG_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const TOG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
  let matches = App::new("tog")
    .version(TOG_VERSION)
    .author(TOG_AUTHOR)
    .about(TOG_DESCRIPTION)
    .arg(Arg::with_name("file")
         .short("f")
         .long("file")
         .value_name("FILE")
         .default_value("tog.yaml")
         .help("Path to the tog profile to be executed")
         .takes_value(true))
    .arg(Arg::with_name("process")
         .short("p")
         .long("process")
         .value_name("PROCESS")
         .help("Specifies a process defined in the tog profile to run individually")
         .takes_value(true))
    .arg(Arg::with_name("daemon")
         .short("D")
         .long("daemon")
         .help("Runs profile as background process"))
    .arg(Arg::with_name("kill")
         .short("k")
         .long("kill")
         .value_name("PID")
         .help("Kills a given process")
         .takes_value(true))
    .arg(Arg::with_name("v")
         .short("v")
         .multiple(true)
         .help("Sets the level of verbosity"))
    .subcommand(SubCommand::with_name("ls")
                .about("Lists currently running tog processes"))
    .get_matches();

  let cmd: Command = get_command(matches);

  run::run(&cmd.profile, cmd.processes_to_run);
}
