# arpx
[![Crates.io](https://img.shields.io/crates/v/arpx?color=black)](https://crates.io/crates/arpx)
[![Build Status](https://travis-ci.com/jaredgorski/arpx.svg?token=7hLupv5JrcFFuyR6Lkp7&branch=master)](https://travis-ci.com/jaredgorski/arpx) 
<br>Automate and relate multiple processes.

## Description

**arpx** runs PROCESSES as they are defined in PROFILES. PROFILES allow for the configuration of PROCESSES, MONITORS, and ACTIONS, which work together to execute commands, watch for triggering conditions, and respond to those triggering conditions with new commands.

PROFILES can be run entirely or a single PROCESS can be run from within a PROFILE. If no PROFILE is defined, **arpx** will look for a file named `arpx.yaml` within the current working directory to run. PROCESSES can be configured to execute alone (blocking), run concurrently, depend on other PROCESSES, and perform ACTIONS when specific conditions are met.

## Usage

### General program information

Command  | Info
-------- | --------
**-h, --help** | Output a usage message and exit.
**-V** | Output the version number of **arpx** and exit.

### Options

Command  | Info
-------- | --------
**-f** PROFILE, **--file**=PROFILE | Execute a PROFILE at the given filepath. Defaults to `./arpx.yaml`.
**-p** PROCESS, **--process**=PROCESS | Execute a single PROCESS from within the current PROFILE.

### Profile configuration
PROFILES can be named `arpx.yaml` or formatted as `<my-prefix>.arpx.yaml`. PROFILES are currently the primary mode of configuration for **arpx** runtimes, at least until a more scriptable/less verbose interface is developed.

```yaml
processes:                          // Define primary PROCESSES.
  - name: [NAME OF PROCESS]         // Add a unique name to identify the PROCESS within the arpx runtime.
    command: [COMMAND]              // The command to execute.
    cwd: [PATH]                     // Directory in which to execute command.
    blocking: [TRUE|FALSE]          // Whether the PROCESS should block the main thread or run concurrently. (default: false)
    silent: [TRUE|FALSE]            // Whether to silence logs for the PROCESS. (default: false)

monitors:                           // Configure MONITORS for specific PROCESSES.
  - process: [NAME OF PROCESS]      // Specify the PROCESS to MONITOR.
    triggers:                       // Define conditions under which MONITOR triggers ACTIONS.
      [TRIGGERS]                    // See `Triggers` section below, under `Monitors`.
    actions:                        // Specify ACTIONS to execute when triggering conditions are met.
      [ACTIONS]                     // See `Actions` section below for an overview on built-in and custom ACTIONS.

actions:                            // Define custom ACTIONS which can be activated by MONITORS under triggering conditions.
  - name: [NAME OF ACTION]          // Add a unique name to identify the ACTION within the arpx runtime.
    type: shell                     // Specify the type of custom ACTION. Currently only `shell` is available, and it must be passed.
    command: [COMMAND]              // The command to execute.
    cwd: [PATH]                     // Directory in which to execute command.
    blocking: [TRUE|FALSE]          // Whether the ACTION should block the main thread or run concurrently. (default: false)
    silent: [TRUE|FALSE]            // Whether to silence logs for the ACTION. (default: false)
```

#### Example profile - script.arpx.yaml:
```yaml
processes:
  - name: loop1
    command: |
      for i in {1..5}
      do
        sleep 1
        echo "Loop1 $i"
      done
    blocking: true
  - name: loop3
    command: |
      for i in {1..5}
      do
        sleep 1
        echo "Loop3 $i"
      done

monitors:
  - process: loop1
    triggers:
      logs:
        includes_string: Loop1 5
    actions:
      - loop2

actions:
  - name: loop2
    type: shell
    command: |
      for i in {1..3}
      do
        sleep 1
        echo "Loop2 $i"
      done
      exit
```

![Example arpx output](https://github.com/jaredgorski/arpx/raw/master/.media/arpx_screenshot-annotated.png)


### Processes
PROCESSES are the primary commands **arpx** will manage. PROCESSES can be run blockingly or concurrently, and can be run one at a time with the `-p` option.

To run a PROCESS named `my-process` contained in a file named `arpx.yaml` in the current working directory, execute:
```shell
$ arpx -p my-process
```

To run all PROCESSES contained in a file named `my.arpx.yaml`, execute:
```shell
$ arpx -f ~/path/to/my.arpx.yaml
```

### Monitors
MONITORS watch for conditions in a given PROCESS and perform ACTIONS if/when those conditions are met. MONITORS are configured by defining a "triggers" struct. The available triggers are specified below.

#### Triggers
- **logs**: reads PROCESS output (stdout/stderr) line-by-line
  - `includes_string`: searches PROCESS output line-by-line for a string

### Actions
ACTIONS are new PROCESSES which can be executed when triggered by a MONITOR. There are built-in actions available to all MONITORS specified below. Custom ACTIONS can also be defined.

#### Built-in
- **exit**: Exit **arpx**.
- **kill**: Exit the current PROCESS.
- **respawn**: Exit and restart the current PROCESS.
- **silence**: Silence logs which trigger the current MONITOR.

#### Custom
Custom ACTIONS can define new tasks to be executed if/when triggering conditions are met. Currently, the only type of ACTION available is `shell`, which allows for defining a shell command to run when the current ACTION is activated.

## Purpose and applications
**arpx** is useful for automatically handling errors, suppressing stdout/stderr output, scheduling processes relative to each other, and more. It was originally concieved for the purpose of automatically handling errors while running a local development environment with multiple dependent services. The goal is that **arpx** should be generally useful, so please open issues for enhancements with a mind toward powerful, general utility.

## Contributing
This project is my first program in Rust. It's rough around the edges and has not been styleguided yet, but "simplicity" and "clarity" are words that come to mind when I envision what it should look like down the road. Contributions and suggestions should reflect the [Unix Philosophy](https://homepage.cs.uri.edu/~thenry/resources/unix_art/ch01s06.html).
