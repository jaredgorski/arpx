extern crate tog;
extern crate clap;
use clap::{Arg, App, SubCommand};
// use tog::commands;

const TOG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const TOG_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const TOG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
  let matches = App::new("tog")
    .version(TOG_VERSION)
    .author(TOG_AUTHOR)
    .about(TOG_DESCRIPTION)
    .arg(Arg::with_name("file")
         .short("f")
         .long("file")
         .value_name("FILE")
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


  if matches.is_present("ls") {
    println!("Listing processes...");
  } else {
    let file = matches.value_of("file").unwrap_or("tog.yaml");
    let process = matches.value_of("process").unwrap_or("no_process");
    let to_kill = matches.value_of("kill").unwrap_or("no_to_kill");

    if matches.is_present("daemon") {
      println!("Will execute in background");
    }

    println!("Killing pid: {}", to_kill);
    println!("Value for file: {}", file);
    println!("Executing process: {}", process);
  }

  match matches.occurrences_of("v") {
    0 => println!("logging level: Info"),
    1 => println!("logging level: Verbose"),
    2 => println!("logging level: Debug"),
    3 | _ => println!("logging level: Silly"),
  }

  // logic
}
