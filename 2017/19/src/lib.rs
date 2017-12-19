pub fn answer_1(_input: &str) -> String {
    String::new()
}

pub fn answer_2(_input: &str) -> u32 {0}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = [
            "     |          ",
            "     |  +--+    ",
            "     A  |  C    ",
            " F---|----E|--+ ",
            "     |  |  |  D ",
            "     +B-+  +--+ ",
        ].join("\n");

        assert_eq!(String::from("ABCDEF"), answer_1(&input));
    }
}
