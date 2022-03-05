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

### Binary distribution
- [link](https://rust-cli.github.io/book/tutorial/packaging.html#distributing-binaries)

### Code quality
- decide on linting settings (make a clippy toml)

### Performance
- benchmarking:
  - [link](https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/bench.html)
  - [link](https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html)

### Feature backlog:
  - optional log monitor ontrigger (just run code on the buffer)
  - external log monitors:
    - use arpx to monitor external log file and spawn processes ontrigger
  - per-process log colors
  - per-process log file
