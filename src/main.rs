#![cfg_attr(feature = "doc", doc(include = "../README.md"))]
//! https://github.com/jaredgorski/arpx
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]

use clap::{App, Arg};

mod action;
mod arpx;
mod error;
mod process;
mod profile;
mod util;

#[doc(hidden)]
pub const APPNAME: &str = "arpx";

#[doc(hidden)]
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[doc(hidden)]
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

#[doc(hidden)]
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[doc(hidden)]
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

#[doc(hidden)]
const DEBUG_FLAG: bool = false;

#[doc(hidden)]
const ERROR_PREFIX: &str = r#"
ERROR:
"#;

fn wrap_arpx_error(error_str: String) -> String {
    format!("{}\n> {}\n\n", ERROR_PREFIX, error_str)
}

fn print_error(error: error::ArpxError) {
    let formatted = match DEBUG_FLAG {
        true => format!("{:?}", error),
        false => format!("{}", error),
    };

    let wrapped = wrap_arpx_error(formatted);

    eprint!("{}", wrapped)
}

#[doc(hidden)]
fn main() {
    let default_profile: String = format!("{}.yaml", APPNAME);
    let matches = App::new(APPNAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .default_value(&default_profile)
                .help("Path to the profile to be executed")
                .takes_value(true),
        )
        .arg(
            Arg::new("process")
                .short('p')
                .long("process")
                .multiple_occurrences(true)
                .value_name("PROCESS")
                .help("Processes in the profile to run (runs all if none given)")
                .takes_value(true),
        )
        .get_matches();

    let requested_profile_file: String =
        matches.value_of("file").unwrap_or("arpx.yaml").to_string();

    let requested_processes: Vec<String> = {
        if matches.is_present("process") {
            matches
                .values_of("process")
                .unwrap()
                .map(|x| x.to_string())
                .collect()
        } else {
            Vec::new()
        }
    };

    let a = arpx::Arpx::new();

    let a_loaded = match a.load_profile(requested_profile_file) {
        Ok(a) => a,
        Err(error) => {
            print_error(error);
            std::process::exit(1);
        }
    };

    match a_loaded.run(requested_processes) {
        Ok(()) => (),
        Err(error) => {
            print_error(error);
            std::process::exit(1);
        }
    };
}
