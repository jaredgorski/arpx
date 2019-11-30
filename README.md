# arpx [![Build Status](https://travis-ci.com/jaredgorski/arpx.svg?token=7hLupv5JrcFFuyR6Lkp7&branch=master)](https://travis-ci.com/jaredgorski/arpx)
Automate and relate multiple processes.

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

```yaml
processes:
  - name: loop
    command: |
      for i in {1..5}
      do
        sleep 1
        echo "Loop $i"
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
  - process: loop
    triggers:
      logs:
        includes_string: Loop 5
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
    blocking: false
```

### Processes

### Monitors

#### Triggers

### Actions

#### Built-in

#### Custom

## Applications

