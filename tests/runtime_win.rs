mod common;

use common::TC;

#[cfg(windows)]
test!(single_job_with_single_task, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: p1;

            processes:
                p1:
                    command: 'echo foo & timeout /t 1 >nul & echo bar & timeout /t 1 >nul & echo baz & timeout /t 1 >nul'
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!(vec!["[p1] foo \r", "[p1] bar \r", "[p1] baz \r"], out[1..4]);
    assert_eq!(5, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(single_job_with_multiple_tasks, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: |
                    p1;
                    p2;

            processes:
                p1:
                    command: 'echo foo & timeout /t 1 >nul'
                p2:
                    command: 'echo bar & timeout /t 1 >nul & echo baz & timeout /t 1 >nul'
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!("[p1] foo \r", out[1]);
    assert_eq!(vec!["[p2] bar \r", "[p2] baz \r"], out[4..6]);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(single_job_with_concurrent_task, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: |
                    [
                        p1;
                        p2;
                    ]

            processes:
                p1:
                    command: 'echo foo & timeout /t 2 >nul & echo baz'
                p2:
                    command: 'timeout /t 1 >nul & echo bar & timeout /t 1 >nul'
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_btw!("foo", out, 0, 3);
    assert_btw!("bar", out, 2, 4);
    assert_btw!("baz", out, 3, 6);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(single_job_with_single_and_concurrent_task, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: |
                    p0;
                    [
                        p1;
                        p2;
                    ]

            processes:
                p0:
                    command: 'echo qux & timeout /t 1 >nul'
                p1:
                    command: 'echo foo & timeout /t 2 >nul & echo baz'
                p2:
                    command: 'timeout /t 1 >nul & echo bar & timeout /t 1 >nul'
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!("[p0] qux \r", out[1]);
    assert_btw!("foo", out, 2, 6);
    assert_btw!("bar", out, 3, 9);
    assert_btw!("baz", out, 6, 9);
    assert_eq!(10, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(multiple_jobs, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test_1: p1;
                test_2: p2;

            processes:
                p1:
                    command: 'echo foo & timeout /t 1 >nul'
                p2:
                    command: 'echo bar & timeout /t 1 >nul & echo baz & timeout /t 1 >nul'
        "#,
        )
        .opts("-j test_1 -j test_2")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!("[p1] foo \r", out[1]);
    assert_eq!(vec!["[p2] bar \r", "[p2] baz \r"], out[4..6]);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(job_with_onsucceed_process, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: one;

            processes:
                one:
                    command: 'echo foo & timeout /t 1 >nul'
                    onsucceed: two
                two:
                    command: 'echo bar & timeout /t 1 >nul'
                    onsucceed: three
                three:
                    command: 'echo baz & timeout /t 1 >nul'
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!("[one] foo \r", out[1]);
    assert_btw!("bar", out, 3, 8);
    assert_btw!("baz", out, 5, 8);
    assert_eq!(9, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(job_with_onfail_process, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: one;

            processes:
                one:
                    command: 'echo foo & timeout /t 1 >nul & exit 1'
                    onfail: two
                two:
                    command: 'echo bar & timeout /t 1 >nul & exit 1'
                    onfail: three
                three:
                    command: 'echo baz & timeout /t 1 >nul'
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!("[one] foo \r", out[1]);
    assert_btw!("bar", out, 3, 8);
    assert_btw!("baz", out, 5, 8);
    assert_eq!(9, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(job_with_single_log_monitor, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: p1; @m1

            processes:
                p1:
                    command: 'echo foo & timeout /t 1 >nul & echo bar & timeout /t 1 >nul'
                p2:
                    command: 'echo baz & timeout /t 1 >nul'
            log_monitors:
                m1:
                    buffer_size: 1
                    test: |
                        echo.%ARPX_BUFFER%|findstr /C:"bar" >nul 2>&1
                    ontrigger: p2
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!(vec!["[p1] foo \r", "[p1] bar \r"], out[1..3]);
    assert_btw!("baz", out, 3, 6);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

#[cfg(windows)]
test!(job_with_multiple_log_monitors, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: p1; @m1 @m2

            processes:
                p1:
                    command: 'echo foo & timeout /t 2 >nul & echo bar & timeout /t 1 >nul'
                p2:
                    command: 'echo baz & timeout /t 1 >nul'
                p3:
                    command: 'timeout /t 3 >nul & echo qux & timeout /t 1 >nul'
            log_monitors:
                m1:
                    buffer_size: 1
                    test: |
                        echo.%ARPX_BUFFER%|findstr /C:"foo" >nul 2>&1
                    ontrigger: p2
                m2:
                    buffer_size: 1
                    test: |
                        echo.%ARPX_BUFFER%|findstr /C:"bar" >nul 2>&1
                    ontrigger: p3
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    debug!(out);
    assert_eq!("[p1] foo \r", out[1]);
    assert_btw!("baz", out, 2, 6);
    assert_btw!("bar", out, 2, 6);
    assert_btw!("qux", out, 7, 9);
    assert_eq!(10, out.len());
    assert_eq!(0, err.len());
});
