use clap::{arg, command, ArgMatches, Command};

#[doc(hidden)]
pub struct Cli;

impl Cli {
    #[must_use]
    pub fn run() -> ArgMatches {
        return command!()
            .propagate_version(true)
            .arg_required_else_help(true)
            .arg(arg!(-f --file <FILE> "Path to profile"))
            .arg(arg!(-j --job <JOB> "Job in profile to run").multiple_occurrences(true))
            .arg(arg!(-v --verbose))
            .arg(arg!(--debug))
            .subcommand(
                Command::new("bin")
                    .about("Local binary on which to invoke process commands")
                    .arg(arg!([BIN]))
                    .arg(
                        arg!(-a --args <BINARGS> "Arguments passed to custom binary prior to process commands")
                            .allow_hyphen_values(true)
                            .multiple_values(true)
                            .required(false),
                    ),
            )
            .get_matches();
    }
}
