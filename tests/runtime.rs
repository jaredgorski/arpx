mod common;

use common::TC;

test!(single_job_with_single_process, |t: TC| {
    let (out, err) = t
        .profile(
            r#"
            jobs:
                test: loop1;

            processes:
                loop1:
                    command: |
                        echo foo
                        echo bar
                        echo baz
        "#,
        )
        .opts("-j test")
        .run()
        .unwrap();

    let expected = vec!["[loop1] foo", "[loop1] bar", "[loop1] baz"];

    assert_eq!(expected, out[1..4]);
    assert_eq!(5, out.len());
    assert_eq!(0, err.len());
});
