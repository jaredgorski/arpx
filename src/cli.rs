use clap::{arg, command, ArgMatches, Command};

pub struct Cli;

impl Cli {
    pub fn run() -> ArgMatches {
        return command!()
            .propagate_version(true)
            .arg_required_else_help(true)
            .arg(arg!(-f --file <FILE> "Path to profile"))
            .arg(arg!(-j --jobs <JOB> "Jobs in profile to run").multiple_values(true))
            .arg(arg!(-v - -verbose))
            .arg(arg!(--debug))
            .subcommand(
                Command::new("bin")
                    .about("Local binary on which to invoke process commands")
                    .arg(arg!([NAME]))
                    .arg(
                        arg!(-a --args <BINARGS> "Arguments passed to custom binary prior to process commands")
                            .multiple_values(true)
                            .required(false),
                    ),
            )
            .get_matches();
    }
}
