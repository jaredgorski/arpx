use std::{
    error,
    fmt::{self, Debug, Display},
};

use crate::REPOSITORY;

pub struct ArpxError(Box<ArpxErrorEnum>);

#[derive(Debug)]
pub enum ArpxErrorEnum {
    ProfileNotFound(String),
    ProfileParseError(String),
    InternalError(String),
    InvalidConfiguration(String),
}

pub(crate) fn profile_not_found(filepath: String) -> ArpxError {
    ArpxError(Box::new(ArpxErrorEnum::ProfileNotFound(filepath)))
}

pub(crate) fn profile_parse_error(error_display: String) -> ArpxError {
    ArpxError(Box::new(ArpxErrorEnum::ProfileParseError(error_display)))
}

pub(crate) fn internal_error(details: String) -> ArpxError {
    ArpxError(Box::new(ArpxErrorEnum::InternalError(details)))
}

pub(crate) fn invalid_configuration(details: String) -> ArpxError {
    ArpxError(Box::new(ArpxErrorEnum::InvalidConfiguration(details)))
}

impl error::Error for ArpxError {
    fn description(&self) -> &str {
        match self.0.as_ref() {
            ArpxErrorEnum::ProfileNotFound(filepath) => filepath,
            ArpxErrorEnum::ProfileParseError(error_display) => error_display,
            ArpxErrorEnum::InternalError(details) => details,
            ArpxErrorEnum::InvalidConfiguration(details) => details,
        }
    }
}

impl Display for ArpxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.as_ref() {
            ArpxErrorEnum::ProfileNotFound(filepath) => write!(
                f,
                "Profile not found at given filepath.\n  Given: {}",
                filepath
            ),
            ArpxErrorEnum::ProfileParseError(error_display) => {
                write!(f, "Could not parse profile.\n  Error: {}", error_display)
            }
            ArpxErrorEnum::InternalError(details) => write!(
                f,
                "Internal error.\n  Details: {}\n\nPlease consider opening an issue at {}/issues.",
                details, REPOSITORY
            ),
            ArpxErrorEnum::InvalidConfiguration(details) => {
                write!(f, "Invalid Arpx configuration.\n  Details: {}", details)
            }
        }
    }
}

impl Debug for ArpxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.as_ref() {
            ArpxErrorEnum::ProfileNotFound(filepath) => {
                f.debug_tuple("Filepath").field(filepath).finish()
            }
            ArpxErrorEnum::ProfileParseError(error_display) => {
                f.debug_tuple("Error").field(error_display).finish()
            }
            ArpxErrorEnum::InternalError(details) => f.debug_tuple("Error").field(details).finish(),
            ArpxErrorEnum::InvalidConfiguration(details) => {
                f.debug_tuple("Error").field(details).finish()
            }
        }
    }
}
