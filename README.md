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
