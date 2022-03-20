use anyhow::{Context, Result};
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};

pub enum Patterns {
    Console,
    Debug,
    File,
}

impl Patterns {
    pub fn as_str(&self) -> &str {
        match self {
            Patterns::Console => "[{h({T})}] {m}{n}",
            Patterns::Debug => "{d} {l} {f}, line {L}: [{T} \\({I}\\)] {m}{n}",
            Patterns::File => "{d} | {l} | {T} \\({I}\\) > {m}{n}",
        }
    }
}

/// Configures runtime logging.
///
/// Once this object is initialized, logging macros are available care of [`log4rs`].
///
/// [`log4rs`]: https://docs.rs/log4rs/latest/log4rs/
pub struct Logs {
    pub handle: Handle,
}

impl Logs {
    /// Initiate runtime logging with the provided level.
    ///
    /// If verbose, use verbose logging pattern.
    pub fn init(level: LevelFilter, verbose: bool) -> Result<Self> {
        let handle = log4rs::init_config(Self::get_config(level, verbose)?)
            .context("Error initiating logger config")?;

        Ok(Self { handle })
    }

    /// Build a logging configuration with the given level and verbosity.
    fn get_config(level: LevelFilter, verbose: bool) -> Result<Config> {
        let pattern = match level {
            LevelFilter::Debug => Patterns::Debug.as_str(),
            _ => {
                if verbose {
                    Patterns::File.as_str()
                } else {
                    Patterns::Console.as_str()
                }
            }
        };

        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build();

        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(level))
            .context("Error building logger config")
    }
}
