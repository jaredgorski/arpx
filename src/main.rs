#![cfg_attr(feature = "doc", doc(include = "../README.md"))]
//! https://github.com/jaredgorski/arpx
#![warn(
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    private_doc_tests,
    trivial_casts,
    trivial_numeric_casts,
    unused_allocation,
    unused_import_braces
)]
#![forbid(unsafe_code)]

mod cli;
mod runtime;
mod util;

use cli::Cli;
use log::{debug, LevelFilter};
use runtime::{local_bin::BinCommand, Runtime};
use util::logs::Logs;

fn main() -> Result<(), std::io::Error> {
    let matches = Cli::run();

    Logs::init(
        if matches.is_present("debug") {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
        matches.is_present("verbose"),
    );

    debug!("CLI returned matches: {:#?}", matches);

    let path = matches.value_of("file").unwrap_or("arpx.yaml").to_string();
    let jobs = matches
        .values_of("job")
        .unwrap()
        .map(std::string::ToString::to_string)
        .collect();

    debug!("Profile path from CLI matches: {}", path);
    debug!("Jobs from CLI matches: {:?}", jobs);
    debug!("Program start");

    let mut runtime = match Runtime::from_profile(&path[..], jobs) {
        Ok(runtime) => runtime,
        Err(error) => panic!("{:?}", error),
    };

    if let Some(("bin", sub_matches)) = matches.subcommand() {
        let bin = sub_matches.value_of("BIN").unwrap();
        let args = match sub_matches.values_of("args") {
            Some(a) => a.map(std::string::ToString::to_string).collect(),
            None => Vec::new(),
        };

        runtime.bin_command(BinCommand::new(bin.into(), args));
    }

    runtime.run()
}
