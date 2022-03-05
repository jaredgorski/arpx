mod common;

use common::TC;

// TODO:
// - profile file doesn't exist
// - invalid job/process/log_monitor name syntax (alphanumeric + - + _)
// - invalid process (onfail/onsucceed doesn't exist, etc.)
// - invalid log_monitor (ontrigger doesn't exist, etc.)

/*
 * jobs
 */
test!(invalid_jobs_yaml, |t: TC| {
    let (out, err) = t.profile("jobs: oops").opts("-j test").run().unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error deserializing file"));
    assert_eq!(
        true,
        err[4].contains("jobs: invalid type: string \"oops\", expected a map")
    );
    assert_eq!(0, out.len());
});

test!(invalid_jobs_yaml_empty, |t: TC| {
    let (out, err) = t.profile("jobs:").opts("-j test").run().unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error deserializing file"));
    assert_eq!(
        true,
        err[4].contains("jobs: invalid type: unit value, expected a map")
    );
    assert_eq!(0, out.len());
});

test!(job_parse_error, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: $oops
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error deserializing file"));
    assert_eq!(true, err[4].contains("Parse error at job line 1 column 0"));
    assert_eq!(0, out.len());
});

test!(job_not_defined, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: foo;

            processes:
                foo:
                    command: echo foo
        "#,
        )
        .opts("-j does_not_exist")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(
        err[4],
        "    1: Requested job \"does_not_exist\" not defined in jobs"
    );
    assert_eq!(0, out.len());
});

test!(job_uses_nonexistent_process, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: does_not_exist;

            processes:
                exists:
                    command: echo foo
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(
        err[4],
        "    1: Job \"test\", task 1: process \"does_not_exist\" not defined in processes"
    );
    assert_eq!(0, out.len());
});

test!(job_uses_nonexistent_log_monitor, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: exists; @does_not_exist

            processes:
                exists:
                    command: echo foo
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(
        err[4],
        "    1: Job \"test\", task 1: log monitor \"does_not_exist\" not defined in log_monitors"
    );
    assert_eq!(0, out.len());
});

/*
 * processes
 */
test!(invalid_processes_yaml, |t: TC| {
    let (out, err) = t.profile("processes: oops").opts("-j test").run().unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error deserializing file"));
    assert_eq!(
        true,
        err[4].contains("processes: invalid type: string \"oops\", expected a map")
    );
    assert_eq!(0, out.len());
});

test!(no_processes_defined, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: does_not_exist;
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(err[4], "    1: No valid processes exist in profile");
    assert_eq!(0, out.len());
});

test!(processes_max, |t: TC| {
    let (out, err) = t
        .env("ARPX_PROCESSES_MAX", "0")
        .profile(
            r#"
            jobs:
                test: foo;

            processes:
                foo:
                    command: echo foo
         "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(err[4], "    1: Too many processes defined in profile");
    assert_eq!(0, out.len());
});

test!(concurrent_processes_max, |t: TC| {
    let (out, err) = t
        .env("ARPX_CONCURRENT_PROCESSES_MAX", "0")
        .profile(
            r#"
            jobs:
                test: |
                    [
                        foo;
                        bar;
                    ]

            processes:
                foo:
                    command: echo foo
                bar:
                    command: echo bar
         "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(err[4], "    1: Job \"test\", task 1: too many processes");
    assert_eq!(0, out.len());
});

/*
 * log_monitors
 */
test!(invalid_log_monitors_yaml, |t: TC| {
    let (out, err) = t
        .profile("log_monitors: oops")
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error deserializing file"));
    assert_eq!(
        true,
        err[4].contains("log_monitors: invalid type: string \"oops\", expected a map")
    );
    assert_eq!(0, out.len());
});

test!(log_monitors_max, |t: TC| {
    let (out, err) = t
        .env("ARPX_LOG_MONITORS_MAX", "0")
        .profile(
            r#"
            jobs:
                test: foo;

            processes:
                foo:
                    command: echo foo

            log_monitors:
                bar:
                    ontrigger: foo
                    test: grep "foo" <<< "$ARPX_BUFFER"
         "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(err[4], "    1: Too many log_monitors defined in profile");
    assert_eq!(0, out.len());
});

/*
 * general
 */
test!(thread_max, |t: TC| {
    let (out, err) = t
        .env("ARPX_THREAD_MAX", "0")
        .profile(
            r#"
            jobs:
                test: foo;

            processes:
                foo:
                    command: echo foo
         "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(true, err[0].contains("Error loading profile"));
    assert_eq!(true, err[3].contains("Error building runtime"));
    assert_eq!(
        err[4],
        "    1: Job \"test\", task 1: too many threads (reduce processes or log_monitors on task)"
    );
    assert_eq!(0, out.len());
});
