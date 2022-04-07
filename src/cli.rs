use clap::{arg, command, ArgMatches};

pub struct Cli;

impl Cli {
    #[must_use]
    pub fn run() -> ArgMatches {
        return command!()
            .propagate_version(true)
            .arg_required_else_help(true)
            .arg(arg!(-f --file <FILE> "Path to profile"))
            .arg(arg!(-j --job <JOB> "Job in profile to run").multiple_occurrences(true))
            .arg(arg!(-v - -verbose))
            .arg(arg!(--debug))
            .get_matches();
    }
}
