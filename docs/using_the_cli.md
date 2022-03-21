# Using the CLI

## Available commands

Command  | Description
-------- | ------------
**-f**, **--file** \<FILE\> | Path to profile
**-j**, **--job** \<JOB\> | Execute job from profile (multiple occurrences are valid)
**-h**, **--help** | Print help information
**-v**, **--verbose** | Enable verbose output
**--debug** | Enable debug output
**-V**, **--version** | Print version information
**bin** \<COMMAND\> **-a** \<ARGS\>... | Customize local binary used to execute process commands (defaults to `sh -c` on MacOS and Linux)

## Usage examples

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
