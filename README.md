![Build Status] [![Latest Version]][crates.io]

[Build Status]: https://travis-ci.com/jaredgorski/arpx.svg?token=7hLupv5JrcFFuyR6Lkp7&branch=master
[Latest Version]: https://img.shields.io/crates/v/arpx?color=black
[crates.io]: https://crates.io/crates/arpx

# arpx

**Small-scale process orchestration**

---

- [Quick demo](https://github.com/jaredgorski/arpx/tree/main/docs/quick_demo.md)
- [Installation instructions](#installing)
- [How to use the CLI](#using-the-cli)
- [How to write a profile](https://github.com/jaredgorski/arpx/tree/main/docs/writing_a_profile.md)

## About

Arpx is a small-scale, run n' gun process orchestrator. In other words, Arpx makes it easy to schedule processes and automate them depending on each others' runtimes in cases where more complex orchestration isn't needed.

Larger-scale process orchestrators (like Kubernetes) allow for in-depth monitoring, complex deployment setups, and granular, real-time process management on top of their normal scheduling and automation features. Arpx, in contrast, aims to provide only that which is necessary for things like running multiple interdependent development servers concurrently, scheduling build scripts in relation to each other, adding naive self-healing to local processes, etc.

> _**a**utomate and **r**elate **p**rocesses(**x**)_

Vaguely, Arpx's primary use-case is development-oriented tasks which variously require scheduling, concurrency, and/or unsophisticated runtime monitoring and handling.

_If you want to hack some orchestration into your development environment, Arpx might be right for you._

### Library vs. binary

The name "Arpx" variously refers to the library which provides the program's core functionality (the Arpx runtime object) as well as the binary which wraps that core functionality in a convenient CLI.

Library-specific documentation can be found on [docs.rs](https://docs.rs/crate/arpx/latest). Documentation in this repository focuses on the Arpx CLI tool.

## Installing

Arpx can be installed using the binaries build on each release or via Rust's [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

### Installing a release binary

1. Navigate to the [Releases](https://github.com/jaredgorski/arpx/releases) page
2. Choose a release to install
3. Download the appropriate binary for your machine
4. Relocate the downloaded binary wherever you'd like it to exist
5. Ensure the location of the binary is in your system's `PATH`

### Installing via Cargo

1. Install Rust on your machine ([docs](https://doc.rust-lang.org/book/ch01-01-installation.html))
2. Execute `cargo install arpx` on your command line

## Using the CLI

Command  | Description
-------- | ------------
**-f**, **--file** \<FILE\> | Path to profile
**-j**, **--job** \<JOB\> | Execute job from profile (multiple occurrences are valid)
**-h**, **--help** | Print help information
**-v**, **--verbose** | Enable verbose output
**--debug** | Enable debug output
**-V**, **--version** | Print version information
**bin** \<COMMAND\> **-a** \<ARGS\>... | Customize local binary used to execute process commands (defaults to `sh -c` on MacOS and Linux)

### Usage examples

Execute job `foo` on `my_profile.yaml`:

```terminal
arpx -f ~/my_profile.yaml -j foo
```

Execute jobs `foo` and `bar` on `my_profile.yaml`:

```terminal
arpx -f ~/my_profile.yaml -j foo -j bar
```

Execute jobs `foo` and `bar` on `my_profile.yaml` using `echo -n` instead of `sh -c`:

```terminal
arpx -f ~/my_profile.yaml -j foo -j bar bin echo -a -n
```

<br/>

#### Note on the philosophy behind this project

<sup>
Arpx is meant to be a duct tape, run n' gun solution for hackily orchestrating program runtimes. Arpx doesn't seek to be a general-purpose, production-ready tool, but feel free to use it however you see fit (in line with <a href="https://github.com/jaredgorski/arpx/blob/main/LICENSE">the license</a>).
</sup>

<br/>

<sub>
If you have ideas for Arpx, please feel free to <a href="https://github.com/jaredgorski/arpx/issues/new/choose">open an issue</a> or <a href="https://jaredgorski.org/about/">contact me directly</a>. I'm happy to discuss this project and any ideas you might have. However, please keep in mind that ideas and feature requests will likely only be implemented if they align well with the aforementioned goals.
</sub>
