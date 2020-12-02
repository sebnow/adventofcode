#[macro_export]
macro_rules! test_example {
    ($name:ident, $solve:expr, $day:literal, $part:literal, $example:literal) => {
        #[test]
        fn $name() {
            let base = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set");
            let file_pattern = format!("day{:02}-{:02}-{:02}", $day, $part, $example);

            let input_filepath = format!("{}/example/{}.txt", base, file_pattern);
            let input = std::fs::read_to_string(&input_filepath)
                .expect(&input_filepath);

            let answer_filepath = format!("{}/example/{}_answer.txt", base, file_pattern);
            let answer = std::fs::read_to_string(&answer_filepath)
                .expect(&answer_filepath);

            assert_eq!(answer.trim(), $solve(&input));
        }
    };
}
