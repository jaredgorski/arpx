mod common;

use common::TC;

// test!(single_job_with_single_task, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: p1;

//             processes:
//                 p1:
//                     command: |
//                         echo foo
//                         echo bar
//                         echo baz
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     let expected = vec!["[p1] foo", "[p1] bar", "[p1] baz"];

//     assert_eq!(expected, out[1..4]);
//     assert_eq!(5, out.len());
//     assert_eq!(0, err.len());
// });

// test!(single_job_with_multiple_tasks, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: |
//                     p1;
//                     p2;

//             processes:
//                 p1:
//                     command: echo foo
//                 p2:
//                     command: |
//                         echo bar
//                         echo baz
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     let p1_expected = vec!["[p1] foo"];
//     let p2_expected = vec!["[p2] bar", "[p2] baz"];

//     assert_eq!(p1_expected, out[1..2]);
//     assert_eq!(p2_expected, out[4..6]);
//     assert_eq!(7, out.len());
//     assert_eq!(0, err.len());
// });

// test!(single_job_with_concurrent_task, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: |
//                     [
//                         p1;
//                         p2;
//                     ]

//             processes:
//                 p1:
//                     command: |
//                         echo foo
//                         sleep 2
//                         echo baz
//                         sleep 1
//                 p2:
//                     command: |
//                         sleep 1
//                         echo bar
//                         sleep 2
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     assert_eq!("[p1] foo", out[2]);
//     assert_eq!("[p2] bar", out[3]);
//     assert_eq!("[p1] baz", out[4]);
//     assert_eq!(7, out.len());
//     assert_eq!(0, err.len());
// });

// test!(single_job_with_single_and_concurrent_task, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: |
//                     p0;
//                     [
//                         p1;
//                         p2;
//                     ]

//             processes:
//                 p0:
//                     command: echo qux
//                 p1:
//                     command: |
//                         echo foo
//                         sleep 2
//                         echo baz
//                         sleep 1
//                 p2:
//                     command: |
//                         sleep 1
//                         echo bar
//                         sleep 2
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     assert_eq!("[p0] qux", out[1]);
//     assert_eq!("[p1] foo", out[5]);
//     assert_eq!("[p2] bar", out[6]);
//     assert_eq!("[p1] baz", out[7]);
//     assert_eq!(10, out.len());
//     assert_eq!(0, err.len());
// });

// test!(multiple_jobs, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test_1: p1;
//                 test_2: p2;

//             processes:
//                 p1:
//                     command: echo foo
//                 p2:
//                     command: |
//                         echo bar
//                         echo baz
//         "#,
//         )
//         .opts("-j test_1 -j test_2")
//         .run()
//         .unwrap();

//     let p1_expected = vec!["[p1] foo"];
//     let p2_expected = vec!["[p2] bar", "[p2] baz"];

//     assert_eq!(p1_expected, out[1..2]);
//     assert_eq!(p2_expected, out[4..6]);
//     assert_eq!(7, out.len());
//     assert_eq!(0, err.len());
// });

// test!(job_with_onsucceed_process, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: one;

//             processes:
//                 one:
//                     command: echo foo
//                     onsucceed: two
//                 two:
//                     command: echo bar
//                     onsucceed: three
//                 three:
//                     command: echo baz
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     assert_eq!("[one] foo", out[1]);
//     assert_eq!("[one] bar", out[4]);
//     assert_eq!("[one] baz", out[7]);
//     assert_eq!(9, out.len());
//     assert_eq!(0, err.len());
// });

// test!(job_with_onfail_process, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: one;

//             processes:
//                 one:
//                     command: |
//                         echo foo
//                         exit 1
//                     onfail: two
//                 two:
//                     command: |
//                         echo bar
//                         exit 1
//                     onfail: three
//                 three:
//                     command: echo baz
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     assert_eq!("[one] foo", out[1]);
//     assert_eq!("[one] bar", out[4]);
//     assert_eq!("[one] baz", out[7]);
//     assert_eq!(9, out.len());
//     assert_eq!(0, err.len());
// });

// test!(job_with_single_log_monitor, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: p1; @m1

//             processes:
//                 p1:
//                     command: |
//                         echo foo
//                         echo bar
//                 p2:
//                     command: echo baz
//             log_monitors:
//                 m1:
//                     buffer_size: 1
//                     test: grep "bar" <<< "$ARPX_BUFFER"
//                     ontrigger: p2
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     let p1_expected = vec!["[p1] foo", "[p1] bar"];

//     assert_eq!(p1_expected, out[1..3]);
//     assert_eq!("[m1] baz", out[5]);
//     assert_eq!(7, out.len());
//     assert_eq!(0, err.len());
// });

// test!(job_with_multiple_log_monitors, |t: TC| {
//     let (out, err) = t
//         .profile(
//             r#"
//             jobs:
//                 test: p1; @m1 @m2

//             processes:
//                 p1:
//                     command: |
//                         echo foo
//                         sleep 2
//                         echo bar
//                 p2:
//                     command: echo baz
//                 p3:
//                     command: |
//                         sleep 2
//                         echo qux
//             log_monitors:
//                 m1:
//                     buffer_size: 1
//                     test: grep "foo" <<< "$ARPX_BUFFER"
//                     ontrigger: p2
//                 m2:
//                     buffer_size: 1
//                     test: grep "bar" <<< "$ARPX_BUFFER"
//                     ontrigger: p3
//         "#,
//         )
//         .opts("-j test")
//         .run()
//         .unwrap();

//     assert_eq!("[p1] foo", out[1]);
//     assert_eq!("[m1] baz", out[3]);
//     assert_eq!("[p1] bar", out[5]);
//     assert_eq!("[m2] qux", out[8]);
//     assert_eq!(10, out.len());
//     assert_eq!(0, err.len());
// });
