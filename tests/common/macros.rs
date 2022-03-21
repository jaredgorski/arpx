#[macro_export]
macro_rules! test {
    ($name:ident, $fn:expr) => {
        #[test]
        fn $name() {
            let test_object = crate::common::TC::new(stringify!($name));

            $fn(test_object);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($input:expr) => {
        assert_eq!(vec![""], $input);
    };
}

#[macro_export]
macro_rules! assert_btw {
    ($expected:expr, $got:expr, $before_index:expr, $after_index:expr) => {
        let after_index = $after_index;
        let before_index = $before_index;
        let expected = &*$expected;
        let got = &*$got;

        let got_index = got
            .iter()
            .position(|item| item.contains(expected))
            .expect(&format!("Expected expression \"{}\" not found in given output", expected)[..]);

        let valid = got_index > before_index && got_index < after_index;

        if !valid {
            panic!("Expected index was not between before and after.\n\tGot: {}\n\tBefore: {}\n\tAfter: {}", got_index, before_index, after_index);
        }
    };
}
