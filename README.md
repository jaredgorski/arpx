# arpx 0.5.0 rewrite

### TODO

- errors:
  - job parsing errors
  - deserialization errors
  - validation errors:
    - protect against thread max
    - process doesn't exist in processes
  - runtime errors
- features:
  - detached monitors:
    - use arpx to monitor external logs, such as a log file
- edge cases:
  - (walk through program from start to finish looking for edge cases)
- code improvements:
  - Reduce `clone()` anti-patterns.
  - Use more references, less values and owned data.
  - Insofar as `ctx` is passed around, the entire `log_monitor_lib` and `process_lib` objects get copied. Can `ctx` be referenced instead?
  - Ensure `pub` is only used where needed.
  - Clearly map out runtime and reason about design strengths/weaknesses.


### Test mapping

- profile:
  - invalid yaml key(s) (job, processes, log_monitors)
  - invalid job yaml
  - invalid job (parse error)
  - invalid job (process doesn't exist, log_monitor doesn't exist, etc.)
  - invalid process yaml
  - invalid process (onfail/onsucceed doesn't exist, etc.)
  - invalid log_monitor yaml
  - invalid log_monitor (ontrigger doesn't exist, etc.)
  - invalid profile configuration (too many threads, etc.)
- runtime:
  - profile file doesn't exist
  - profile file read error (not errors in parsing / deserialization)
  - single job
  - multiple jobs
  - job with single task
  - job with multiple tasks
  - job with concurrent tasks
  - job with serial and concurrent tasks
  - job with recursive process
  - job with single log_monitor
  - job with multiple log_monitors
