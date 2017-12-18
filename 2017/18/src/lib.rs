pub fn answer_1(input: &str) -> i32 {
    0
}

pub fn answer_2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = [
            "set a 1",
            "add a 2",
            "mul a a",
            "mod a 5",
            "snd a",
            "set a 0",
            "rcv a",
            "jgz a -1",
            "set a 1",
            "jgz a -2",
        ].join("\n");
        assert_eq!(4, answer_1(&input));
    }
}
