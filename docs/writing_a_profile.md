# Writing a profile

Arpx runtimes are configured via profiles. Profiles are written using the [YAML spec](https://yaml.org/spec/).

A profile is composed of three items: `jobs`, `processes`, and `log_monitors`. A profile must contain at least one process and one job (to execute that process) to be valid.

## Jobs

The `jobs` key in an Arpx profile is a mapping of string values. For each entry in the `jobs` mapping, the key is the job's name and the value is the job itself, written in the dedicated arpx_job scripting language.

### arpx_job scripting language

The arpx_job scripting language seeks to express Arpx runtime jobs as succinctly as possible and enable users to easily construct and execute jobs from available processes and log monitors.

The arpx_job scripting language can be broken down into 5 concepts:

- **Processes** (`my_process`, `my_other_process`)
  - Any process defined in the current profile can be referenced by name from within arpx_job. For example, if a process named `foo` is defined under `processes`, it can be invoked within a job using its name, "foo". A semicolon must terminate the process declaration. (`foo;`, not `foo`)
- **Concurrency** (`[]`)
  - Multiple processes can be executed in parallel by enclosing their declarations with square brackets. Each process must be terminated with a semicolon. (`[ foo; bar; baz ]`)
- **Contingency** (`?:`)
  - Actions can be executed when a process succeeds or fails using [ternary syntax](https://en.wikipedia.org/wiki/%3F:). `?` denotes an "onsucceed" branch and `:` denotes an "onfail" branch. When contingency is used, the terminating semicolon goes at the end of the entire declaration. (`foo ? bar : baz;`)
- **Actions** (`my_process`, `my_other_process` + `arpx_exit`, `arpx_exit_error`)
  - "Actions" is a supercategory which includes all processes defined in the current profile as well as special system actions `arpx_exit` and `arpx_exit_error`. `arpx_exit` exits the entire Arpx runtime with a successful status. `arpx_exit_error` exits the entire Arpx runtime with a failing status.
- **Log monitors** (`@my_log_monitor`)
  - Any log monitor defined in the current profile can be referenced by name from within arpx_job and applied to a given process declaration by placing it _after the terminating semicolon_. For example, if a log monitor named `qux` is defined under `log_monitors`, it can be applied to a process declaration like so: `foo ? bar : baz; @qux`. Log monitor declarations are always placed after the terminating semicolon.

Each job defined below demonstrates one or more of the concepts described above.

```yaml
jobs:
  sequence: |
    process1;
    process2;
    
  concurrent: |
    [
      process1;
      process2;
    ]
    
  sequence_and_concurrent: |
    process1;
    process2;
    [
      process1;
      process2;
    ]
    
  contingent: process1 ? process2 : process3;
  
  contingent_onsucceed: process1 ? process2;
  
  contingent_onfail: process1 : process2;
  
  with_log_monitors: |
    process1; @monitor1
    process2 ? process3; @monitor2
    
  concurrent_with_log_monitors: |
    [
      process1; @monitor1
      process2 ? process3; @monitor2
    ]
```

To learn more, check out the [`arpx_job_parser` repo](https://github.com/jaredgorski/arpx_job_parser).

## Processes

The `processes` key in an Arpx profile is a mapping of process configuration objects. For each entry in the `processes` mapping, the key is the process's name and the value is the process configuration object.

```yaml
processes:
  example_process:
    command: echo "Hello, World!"             # (required) Command to execute.
    cwd: /directory/in/which/to/run/command   # (optional) Path to directory in which `command` should execute. Defaults to `.`.
    onsucceed: some_action_name               # (optional) Default onsucceed action. Can be overridden in job script. Defaults to none.
    onfail: some_action_name                  # (optional) Default onfail action. Can be overridden in job script. Defaults to none.
```

## Log monitors

The `log_monitors` key in an Arpx profile is a mapping of log monitor configuration objects. For each entry in the `log_monitors` mapping, the key is the log monitor's name and the value is the log monitor configuration object.

```yaml
log_monitors:
  example_log_monitor:
    test: '[[ "$ARPX_BUFFER" =~ "Hello" ]]'   # (required) Test script to execute on each buffer update.
    ontrigger: some_action_name               # (optional) Default ontrigger action. Can be overridden in job script. Defaults to none.
    buffer_size: 1                            # (optional) Size of rolling buffer. Defaults to 20.
```
