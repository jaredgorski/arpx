[![Build Status]][builds] [![Latest Version]][crates.io]

[Build Status]: https://github.com/jaredgorski/arpx/actions/workflows/build-and-test.yml/badge.svg
[builds]: https://github.com/jaredgorski/arpx/actions/workflows/build-and-test.yml
[Latest Version]: https://img.shields.io/crates/v/arpx?color=black
[crates.io]: https://crates.io/crates/arpx

# arpx

**Small-scale process orchestration**

---

- [Quick demo](https://github.com/jaredgorski/arpx/tree/main/docs/quick_demo.md)
- [Installation instructions](#installing)
- [How to use the CLI](https://github.com/jaredgorski/arpx/tree/main/docs/using_the_cli.md)
- [How to write a profile](https://github.com/jaredgorski/arpx/tree/main/docs/writing_a_profile.md)

## About

Arpx is a small-scale, run n' gun process orchestrator. In other words, Arpx makes it easy to schedule processes and automate them depending on each others' runtimes in cases where more complex orchestration isn't needed.

Larger-scale process orchestrators (like Kubernetes) allow for in-depth monitoring, complex deployment setups, and granular, real-time process management on top of their normal scheduling and automation features. Arpx, in contrast, aims to provide only that which is necessary for things like running multiple interdependent development servers concurrently, scheduling build scripts or tests in relation to each other, adding naive self-healing to local processes, etc.

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
2. Choose a release
3. Download the appropriate archive for your machine
4. Unpack the archive and relocate the binary to your desired location
5. Ensure the binary is located in your system's `PATH`
6. Verify that `arpx --version` works on your command line

### Installing via Cargo

1. Install Rust on your machine ([docs](https://www.rust-lang.org/tools/install))
2. Execute `cargo install arpx` on your command line

<br/>

<div align="center">&sect;</div>

<br/>

#### Note on the philosophy behind this project

<sup>
Arpx is meant to be a duct tape, run n' gun solution for hackily orchestrating program runtimes. Arpx doesn't seek to be a general-purpose, production-ready tool, but feel free to use it however you see fit (in line with <a href="https://github.com/jaredgorski/arpx/blob/main/LICENSE">the license</a>).
</sup>

<br/>

<sub>
If you have ideas for Arpx, please feel free to <a href="https://github.com/jaredgorski/arpx/issues/new/choose">open an issue</a> or <a href="https://jaredgorski.org/about/">contact me directly</a>. I'm happy to discuss this project and any ideas you might have. However, please keep in mind that ideas and feature requests will likely only be implemented if they align well with the aforementioned goals.
</sub>
