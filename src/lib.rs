//! A library for building Arpx runtimes. This library provides an interface for constructing Arpx
//! runtimes with code or via a profile.
//!
//! # Anatomy of a runtime
//!
//! The basic anatomy of a typical runtime is as follows:
//!
//! ```text
//! +---------+
//! | Runtime |
//! +---------+
//!    |
//!    |  +------------+
//!    +->| Job: "dev" |
//!       +------------+
//!          |
//!          |  +------+
//!          +->| Task |
//!             +------+
//!                |
//!                |    +---------------------+
//!                +--->| Process: "database" |
//!                |    +---------------------+
//!                |               |
//!                |           +-------+       +-----------+
//!                |           | Fail? |------>| Self heal |
//!                |           +-------+       +-----------+
//!                |
//!                |    +----------------+
//!                +--->| Process: "api" |
//!                     +----------------+
//!                                |
//!                            +-------+       +-----------+
//!                            | Fail? |------>| Self heal |
//!                            +-------+       +-----------+
//! ```
//!
//! A runtime contains one or more jobs, each of which contains one or more tasks, each of which
//! contains one or more processes. Multiple processes in a single task will run concurrently, and
//! any `onsucceed` or `onfail` actions will be performed on the same thread as the process which
//! spawned them.
//!
//! Multiple tasks in a single job will execute in series, in the order in which they're defined on
//! the job object. The same is true for multiple jobs in a single runtime.
//!
//! In the above diagram, the runtime contains one job (`dev`). `dev` contains one task with
//! multiple processes, `database` and `api`. These processes will run concurrently. Both of these
//! processes declare an `onfail` action which performs some sort of self-healing procedure and,
//! presumably, respawns the failed process.
//!
//! So, runtimes can run processes concurrently or in series, as well as respond to success and
//! failure states upon process exit.
//!
//! # Define a runtime using this library
//!
//! A runtime similar to the one diagrammed above can be built with code:
//!
//! ```
//! use arpx::{Job, LogMonitor, Process, Runtime, Task};
//! use std::collections::HashMap;
//!
//! let processes = vec![
//!     Process::new("database".to_string())
//!         .command("run.sh".to_string())
//!         .cwd("/path/to/project/database/".to_string())
//!         .onsucceed(Some("db_recover".to_string())),
//!     Process::new("api".to_string())
//!         .command("run.sh".to_string())
//!         .cwd("/path/to/project/api/".to_string())
//!         .onsucceed(Some("api_recover".to_string())),
//! ];
//!
//! let mut process_map = processes
//!     .clone()
//!     .into_iter()
//!     .map(|process| (process.name.clone(), process))
//!     .collect::<HashMap<String, Process>>();
//!
//! process_map.insert(
//!     "db_recover".to_string(),
//!     Process::new("db_recover".to_string())
//!         .command("self-heal.sh".to_string())
//!         .cwd("/path/to/project/database/".to_string())
//!         .onsucceed(Some("database".to_string()))
//!         .onfail(Some("arpx_exit_error".to_string()))
//! );
//!
//! process_map.insert(
//!     "api_recover".to_string(),
//!     Process::new("api_recover".to_string())
//!         .command("self-heal.sh".to_string())
//!         .cwd("/path/to/project/api/".to_string())
//!         .log_monitors(vec!["db_permissions_error".to_string()])
//!         .onsucceed(Some("api".to_string()))
//!         .onfail(Some("arpx_exit_error".to_string())),
//! );
//!
//! let mut log_monitor_map = HashMap::new();
//!
//! log_monitor_map.insert(
//!     "db_permissions_error".to_string(),
//!     LogMonitor::new("db_permissions_error".to_string())
//!         .buffer_size(1)
//!         .test("echo \"$ARPX_BUFFER\" | grep -q \"Access denied for user\"".to_string())
//!         .ontrigger("arpx_exit_error".to_string())
//! );
//!
//! let jobs = vec![Job::new(
//!     "dev".to_string(),
//!     vec![Task::new(processes)],
//! )];
//!
//! Runtime::new()
//!     .jobs(jobs)
//!     .process_map(process_map)
//!     .log_monitor_map(log_monitor_map)
//!     .run();
//! ```
//!
//! ## About log monitors
//!
//! Note in the above example that the `api_recover` process is provided with a list of
//! `log_monitors`. The runtime isn't limited to handling exit codes; it can be configured to
//! respond to runtime errors as well.
//!
//! Log monitors allow for string matching against a rolling buffer of a given process's output.
//! For example, the `db_permissions_error` log monitor is applied to the `database` process in job
//! `dev`. The `db_permisions_error` log monitor will keep a rolling buffer of size 1 (the 1 most
//! recent line of the process it's watching) and run its `test` script on every push to the
//! buffer. If the script returns with a `0` exit status, the `ontrigger` action will run. In the
//! case of `db_permissions_error`, a successful `test` will exit the entire runtime.
//!
//! # Define a runtime using a profile
//!
//! This runtime can also be defined in a profile:
//!
//! ```yaml
//! jobs:
//!     dev: |
//!         [
//!             database : db_recover; @db_permissions_error
//!             api : api_recover;
//!         ]
//!
//! processes:
//!     database:
//!         command: run.sh
//!         cwd: /path/to/project/database/
//!         onfail: db_recover
//!     db_recover:
//!         command: self-heal.sh
//!         cwd: /path/to/project/database/
//!         onsucceed: database
//!         onfail: arpx_exit_error
//!     api:
//!         command: run.sh
//!         cwd: /path/to/project/api/
//!         onfail: api_recover
//!     api_recover:
//!         command: self-heal.sh
//!         cwd: /path/to/project/api/
//!         onsucceed: api
//!         onfail: arpx_exit_error
//!         
//! log_monitors:
//!     db_permissions_error:
//!         buffer_size: 1
//!         test: 'echo "$ARPX_BUFFER" | grep -q "Access denied for user"'
//!         ontrigger: arpx_exit_error
//! ```
//!
//! A runtime object can be built from this profile by loading the profile using
//! [`Runtime.from_profile()`].
//!
//! [`Runtime.from_profile`]: Runtime#method.from_profile
//!
//! ## The advantage of using a profile
//!
//! The advantage of using a profile is expressiveness. The `process_map` and `log_monitor_map` are
//! defined nicely using YAML maps and the details of how each job orchestrates the available
//! processes and log monitors into an effective runtime are expressed succinctly using the
//! dedicated arpx_job scripting language:
//!
//! ```text
//! [
//!     database : db_recover; @db_permissions_error
//!     api : api_recover;
//! ]
//! ```
//!
//! ### Some syntax notes
//!
//! Because these processes are enclosed in square brackets (`[`, `]`), the runtime knows to group
//! them together in the same task. This means that `database` and `api` will be executed
//! concurrently, as they should.
//!
//! `database` is followed by `: db_recover`. This is an `onfail` declaration. The runtime will
//! parse this and know that it needs to run `db_recover` if the `database` process exits with a
//! non-zero code.
//!
//! If `database` were followed by `? db_recover` instead, `db_recover` would run `onsucceed`
//! instead of `onfail`. If the declaration were `database ? db_recover : db_recover`, then
//! `db_recover` would run no matter what. Note that this is [ternary
//! operator](https://en.wikipedia.org/wiki/%3F:) syntax.
//!
//! Also, the `@db_permissions_error` declaration on the `database` task tells the runtime to apply
//! the `db_permissions_error` log monitor to the `database` process.

mod logs;
mod runtime;

pub use logs::Logs;
pub use runtime::{
    ctx::Ctx,
    job::{
        task::{log_monitor::LogMonitor, process::Process, Task},
        Job,
    },
    local_bin::BinCommand,
    Runtime,
};
