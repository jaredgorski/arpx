mod common;

use common::TC;

test!(single_job_with_single_task, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: p1;

            processes:
                p1:
                    command: |
                        echo foo
                        sleep 0.01
                        echo bar
                        sleep 0.01
                        echo baz
                        sleep 0.01
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(vec!["[p1] foo", "[p1] bar", "[p1] baz"], out[1..4]);
    assert_eq!(5, out.len());
    assert_eq!(0, err.len());
});

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
                    command: |
                        echo foo
                        sleep 0.01
                p2:
                    command: |
                        echo bar
                        sleep 0.01
                        echo baz
                        sleep 0.01
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!("[p1] foo", out[1]);
    assert_eq!(vec!["[p2] bar", "[p2] baz"], out[4..6]);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

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
                    command: |
                        echo foo
                        sleep 0.02
                        echo baz
                p2:
                    command: |
                        sleep 0.01
                        echo bar
                        sleep 0.01
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_between!("[p1] foo", out, 1, 3);
    assert_between!("[p2] bar", out, 2, 4);
    assert_between!("[p1] baz", out, 3, 6);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

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
                    command: echo qux
                p1:
                    command: |
                        echo foo
                        sleep 0.02
                        echo baz
                p2:
                    command: |
                        sleep 0.01
                        echo bar
                        sleep 0.01
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!("[p0] qux", out[1]);
    assert_between!("[p1] foo", out, 3, 6);
    assert_between!("[p2] bar", out, 4, 9);
    assert_between!("[p1] baz", out, 6, 9);
    assert_eq!(10, out.len());
    assert_eq!(0, err.len());
});

test!(multiple_jobs, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test_1: p1;
                test_2: p2;

            processes:
                p1:
                    command: |
                        echo foo
                        sleep 0.01
                p2:
                    command: |
                        echo bar
                        sleep 0.01
                        echo baz
                        sleep 0.01
        "#,
        )
        .opts("-j test_1 -j test_2")
        .run()
        .unwrap();


    assert_eq!("[p1] foo", out[1]);
    assert_eq!(vec!["[p2] bar", "[p2] baz"], out[4..6]);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

test!(job_with_onsucceed_process, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: one;

            processes:
                one:
                    command: echo foo
                    onsucceed: two
                two:
                    command: echo bar
                    onsucceed: three
                three:
                    command: echo baz
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!("[one] foo", out[1]);
    assert_eq!("[one] bar", out[4]);
    assert_eq!("[one] baz", out[7]);
    assert_eq!(9, out.len());
    assert_eq!(0, err.len());
});

test!(job_with_onfail_process, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: one;

            processes:
                one:
                    command: |
                        echo foo
                        exit 1
                    onfail: two
                two:
                    command: |
                        echo bar
                        exit 1
                    onfail: three
                three:
                    command: echo baz
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!("[one] foo", out[1]);
    assert_eq!("[one] bar", out[4]);
    assert_eq!("[one] baz", out[7]);
    assert_eq!(9, out.len());
    assert_eq!(0, err.len());
});

test!(job_with_single_log_monitor, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: p1; @m1

            processes:
                p1:
                    command: |
                        echo foo
                        sleep 0.01
                        echo bar
                        sleep 0.01
                p2:
                    command: |
                        echo baz
                        sleep 0.01
            log_monitors:
                m1:
                    buffer_size: 1
                    test: grep "bar" <<< "$ARPX_BUFFER"
                    ontrigger: p2
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!(vec!["[p1] foo", "[p1] bar"], out[1..3]);
    assert_between!("[m1] baz", out, 3, 6);
    assert_eq!(7, out.len());
    assert_eq!(0, err.len());
});

test!(job_with_multiple_log_monitors, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: p1; @m1 @m2

            processes:
                p1:
                    command: |
                        echo foo
                        sleep 0.02
                        echo bar
                        sleep 0.01
                p2:
                    command: |
                        echo baz
                        sleep 0.01
                p3:
                    command: |
                        sleep 0.03
                        echo qux
            log_monitors:
                m1:
                    buffer_size: 1
                    test: grep "foo" <<< "$ARPX_BUFFER"
                    ontrigger: p2
                m2:
                    buffer_size: 1
                    test: grep "bar" <<< "$ARPX_BUFFER"
                    ontrigger: p3
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    assert_eq!("[p1] foo", out[1]);
    assert_between!("[m1] baz", out, 2, 6);
    assert_between!("[p1] bar", out, 2, 6);
    assert_between!("[m2] qux", out, 7, 9);
    assert_eq!(10, out.len());
    assert_eq!(0, err.len());
});
