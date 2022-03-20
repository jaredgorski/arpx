# arpx 0.5.0 rewrite

### TODO

1. write README
2. write CONTRIBUTING.md
3. write RELEASE.md
4. record feature backlog
5. [save benchmark baseline](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html)
6. record platforms issue in a GitHub issue
7. record code improvements _somewhere_

### Feature backlog:
- optional log monitor ontrigger (just run code on the buffer)
- external log monitors:
  - use arpx to monitor external log file and spawn processes ontrigger
- per-process log colors
- per-process log file
- windows support pending [this issue](https://github.com/rust-lang/rust/issues/92939)
- enable configuring bin and binargs via profile:
  - bash: `sh -c`
  - cmd: `cmd /c`
  - powershell: `powershell -Command`

### Platforms:
- Currently only supporting x86 MacOS and Linux:
  - Work to fix tests for different platforms (notably i686 and Windows)
  - windows support pending [this issue](https://github.com/rust-lang/rust/issues/92939)
  - Use os_string where needed?

### Code improvements:
- Review ownership: reduce `clone()` anti-patterns.
- Use more references, less values and owned data.
- Insofar as `ctx` is passed around, the entire `log_monitor_map` and `process_map` objects get copied. Can `ctx` be referenced instead?
- Ensure `pub` is only used where needed.
- Clearly map out runtime and reason about design strengths/weaknesses.


---

![Build Status] [![Latest Version]][crates.io]

[Build Status]: https://travis-ci.com/jaredgorski/arpx.svg?token=7hLupv5JrcFFuyR6Lkp7&branch=master
[Latest Version]: https://img.shields.io/crates/v/arpx?color=black
[crates.io]: https://crates.io/crates/arpx

# arpx

**Small-scale process orchestration**

## About

Arpx is a small-scale, run n' gun-style process orchestrator. In other words, Arpx makes it easy to schedule processes and automate them depending on each others' runtimes in cases where more complex orchestration isn't needed.

Larger-scale process orchestrators (like Kubernetes) allow for in-depth monitoring, complex deployment setups, and granular, real-time process management on top of their normal scheduling and automation features. Arpx, in contrast, aims to provide only that which is necessary for things like running multiple concurrent, interdependent development servers, scheduling build scripts, adding naive self-healing to local processes, etc.

> _**a**utomate and **r**elate **p**rocesses(**x**)_

Vaguely, Arpx's primary use-case is development-oriented tasks which variously require scheduling, concurrency, and/or unsophisticated runtime monitoring and handling.

_If you want to hack some orchestration into your development environment, Arpx might be right for you._

### Library vs. binary

The name "Arpx" variously refers to the library which provides the program's core functionality (the Arpx runtime object) as well as the binary which wraps that core functionality in a convenient CLI.

Library-specific documentation can be found on [docs.rs](https://docs.rs/crate/arpx/latest). The rest of this README provides a more general overview of how Arpx works and how to use the CLI.

## Quick demo

1. Download the correct Arpx binary for your operating system and place it in your `PATH` (or equivalent) so that it can be executed by name from your command line.
2. Create a new file somewhere on your computer called `arpx_demo.yaml`.
3. In `arpx_demo.yaml`, paste this text:

    ```yaml
    jobs:
      foo: |
        bar ? baz : qux;
        [
          bar;
          baz;
          qux;
        ]
        bar; @quux

    processes:
      bar:
        command: echo bar
      baz:
        command: echo baz
      qux:
        command: echo qux
      quux:
        command: echo quux

    log_monitors:
      quux:
        buffer_size: 1
        test: 'echo "$ARPX_BUFFER" | grep -q "bar"' # or equivalent for your system
        ontrigger: quux
    ```

4. In your terminal, execute:

    ```terminal
    arpx -f /path/to/arpx_demo.yaml -j foo
    ```
    
5. If you did everything right, you should see _something like_ the following output in your terminal:

    ```terminal
    [bar] "bar" (1) spawned
    [bar] bar
    [bar] "bar" (1) succeeded
    [bar] "baz" (2) spawned
    [bar] baz
    [bar] "baz" (2) succeeded
    [bar] "bar" (3) spawned
    [baz] "baz" (4) spawned
    [qux] "qux" (5) spawned
    [bar] bar
    [baz] baz
    [qux] qux
    [bar] "bar" (3) succeeded
    [baz] "baz" (4) succeeded
    [qux] "qux" (5) succeeded
    [bar] "bar" (6) spawned
    [bar] bar
    [bar] "bar" (6) succeeded
    [quux] "quux" (7) spawned
    [quux] quux
    [quux] "quux" (7) succeeded
    ```

Let's break this down.

Job `foo` contains three tasks:

1. `bar ? baz : qux;`
2. `[ bar; baz; qux; ]`
3. `bar; @quux`

### Task 1: contingency

```text
bar ? baz : qux;
```

The Arpx runtime can be programmed to respond to process exit statuses using ternary syntax. If the initial process succeeds, the `?` branch runs. If the initial process fails, the `:` branch runs. In this case, the runtime will execute `baz` when `bar` exits with a successful status.

Contingency only works on one level for now, so ternary operators can't be chained. Chaining will result in a parsing error.

### Task 2: concurrency

```text
[
  bar;
  baz;
  qux;
]
```

Any given job in an Arpx runtime is composed of tasks. Each task represents one or more _concurrent_ processes. Multiple processes can be programmed into a single task by enclosing with square brackets. When more than one process is enclosed in square brackets, those processes will run simultaneously.

**Note:** Contingency and log monitor declarations can be included in each process declaration, so this is a valid task:

```text
[
  bar ? baz;
  qux : bar;
  baz ? baz : qux; @quux;
]
```

### Task 3: a log monitor

```text
bar; @quux
```

This runtime job task contains a log monitor declaration (`@quux`). This means that the log monitor named `quux`, defined in the `log_monitors` mapping on the profile, will run concurrently with `bar` and watch its output, storing its most recent _n_ number of lines in a rolling buffer of _n_ size. The buffer size is set to `1` in this case, but it defaults to `20`.

With each update to the buffer, the log monitor will run its `test` script. The `test` script has access to a local environment variable called `ARPX_BUFFER` which it can use to string match for certain program conditions visible via the process logs. If the `test` script returns with a `0` status and there is an `ontrigger` action defined for the log monitor, the `ontrigger` action will be executed.

For example, a given process may log a 14 line long error message. A log monitor with a `buffer_size` of `14` can be used to match against that error message and respond to the error state during runtime. When the log monitor matches the error output, it will execute its `ontrigger` action. For a list of available actions, TODO.

Log monitors can be defined without an `ontrigger` action as well, in which case the log monitor will still execute the `test` script on each update to the buffer. This opens up the possibility of using log monitors to append external log files and otherwise respond to log states within the `test` script itself.

For example, the following log monitor exists solely to append process output to a log file:

```text
jobs:
  job1: proc1; @mon1
  
...

log_monitors:
  mon1:
    buffer_size: 1
    test: 'echo "$ARPX_BUFFER" >> /path/to/test.log'
```

### Putting it all together

When our profile is loaded and executed with Arpx, the following happens:

1. Task 1 begins. Process `bar` is executed and successfully exits.
2. Because `bar` exited successfully, the Arpx runtime executes process `baz`. This concludes task 1.
3. Task 2 begins. Processes `bar`, `baz`, and `qux` are spawned simultaneously in separate threads.
4. `bar`, `baz`, and `qux` all exit successfully. This concludes task 2.
5. Task 3 begins. Process `bar` is spawned and the log monitor `quux` is spawned alongside it, receiving its output and storing it in a buffer.
6. `bar` logs "bar" to stdout and `quux` receives it, running its `test` script against the text. `test` exits successfully, so the Arpx runtime executes `quux`'s `ontrigger` action, also called `quux`.
7. Process `quux` is executed and successfully exits. This is the end of the Arpx runtime.

## Using the CLI

Command  | Info
-------- | --------
**-f**, **--file** <FILE> | Path to profile
**-j**, **--job** <JOB> | Execute job from profile (multiple occurrences are valid)
**-h**, **--help** | Print help information
**-v**, **--verbose** | Enable verbose output
**--debug** | Enable debug output
**-V**, **--version** | Print version information
**bin <COMMAND> -a <ARGS>...** | Customize local binary used to execute process commands. Defaults to `sh -c` on MacOS and Linux.

### Examples

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

## Writing a profile

Arpx runtimes are configured via profiles. Profiles are written using the [YAML spec](https://yaml.org/spec/).

A profile is composed of three items: `jobs`, `processes`, and `log_monitors`. A profile must contain at least one process and one job (to execute that process) to be valid.

### Jobs

The `jobs` key in an Arpx profile is a mapping of string values. For each entry in the `jobs` mapping, the key is the job's name and the value is the job itself, written in the dedicated arpx_job scripting language.

#### arpx_job scripting language

#### Examples

### Processes

The `processes` key in an Arpx profile is a mapping of process configuration objects. For each entry in the `processes` mapping, the key is the process's name and the value is the process configuration object.

### Log monitors

The `processes` key in an Arpx profile is a mapping of log monitor configuration objects. For each entry in the `log_monitors` mapping, the key is the log monitor's name and the value is the log monitor configuration object.
