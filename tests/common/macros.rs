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
