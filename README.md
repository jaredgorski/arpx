# arpx 0.5.0 rewrite

### TODO

- edge cases:
  - (walk through program from start to finish looking for edge cases)
- code improvements:
  - Review ownership: reduce `clone()` anti-patterns.
  - Use more references, less values and owned data.
  - Insofar as `ctx` is passed around, the entire `log_monitor_lib` and `process_lib` objects get copied. Can `ctx` be referenced instead?
  - Ensure `pub` is only used where needed.
  - Clearly map out runtime and reason about design strengths/weaknesses.
- write documentation
- add changelog

### Test fixes:
- Tests are inconsistent...

### Platforms:
- Currently only supporting x86 MacOS and Linux
- Work to fix tests for different platforms (notably i686 and Windows)
- Use os_string where needed?

### Feature backlog:
- optional log monitor ontrigger (just run code on the buffer)
- external log monitors:
  - use arpx to monitor external log file and spawn processes ontrigger
- per-process log colors
- per-process log file
