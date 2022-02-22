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
  - monitors:
    - configure rolling buffer size
    - allow for matching against a given process's current rolling buffer on each change to the buffer
  - detached monitors:
    - use arpx to monitor external logs, such as a log file
- edge cases:
  - (walk through program from start to finish looking for edge cases)
