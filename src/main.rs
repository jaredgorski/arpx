use anyhow::{Context, Result};
use arpx::{BinCommand, Cli, Logs, Runtime};
use log::{debug, LevelFilter};

fn main() -> Result<()> {
    let matches = Cli::run();

    Logs::init(
        if matches.is_present("debug") {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
        matches.is_present("verbose"),
    )?;

    debug!("CLI returned matches: {:#?}", matches);

    let path = match matches.value_of("file") {
        Some(f) => f,
        _ => "arpx.yaml",
    };
    let jobs = match matches.values_of("job") {
        Some(jobs) => jobs.map(std::string::ToString::to_string).collect(),
        None => Vec::new(),
    };

    debug!("Profile path from CLI matches: {}", path);
    debug!("Jobs from CLI matches: {:?}", jobs);
    debug!("Program start");

    let mut runtime =
        Runtime::from_profile(path, &jobs).context(format!("Error loading profile at {}", path))?;

    if let Some(("bin", sub_matches)) = matches.subcommand() {
        let bin = sub_matches.value_of("BIN");
        let args = match sub_matches.values_of("args") {
            Some(a) => a.map(std::string::ToString::to_string).collect(),
            None => Vec::new(),
        };

        if let Some(cmd) = bin {
            runtime.bin_command(BinCommand::new(cmd.into(), args));
        }
    }

    runtime.run()
}
