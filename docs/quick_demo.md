# Quick demo

## Instructions

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

## What did we just do?

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
2. Because `bar` exited successfully, the Arpx runtime executes `baz`. This concludes task 1.
3. Task 2 begins. Processes `bar`, `baz`, and `qux` are spawned simultaneously in separate threads.
4. `bar`, `baz`, and `qux` all exit successfully. This concludes task 2.
5. Task 3 begins. Process `bar` is spawned and the log monitor `quux` is spawned alongside it, receiving its output and storing it in a buffer.
6. `bar` logs "bar" to stdout and `quux` receives it, running its `test` script against the text. `test` exits successfully, so the Arpx runtime executes `quux`'s `ontrigger` action, which is a process also named `quux`.
7. Process `quux` is executed and successfully exits. This is the end of the Arpx runtime.
