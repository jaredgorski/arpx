//! A library for building Arpx runtimes. This library provides an interface for constructing Arpx
//! runtimes manually using code or via a profile.

mod logs;
mod runtime;

pub use logs::Logs;
pub use runtime::{
    ctx::Ctx,
    job::{
        task::{process::Process, Task},
        Job,
    },
    local_bin::BinCommand,
    Runtime,
};
