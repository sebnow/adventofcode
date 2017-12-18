#[macro_use]
extern crate failure;

mod instr;

use std::str::FromStr;

use instr::*;

fn parse_input(input: &str) -> Vec<Instr> {
    input.lines().map(|l| Instr::from_str(l).unwrap()).collect()
}

pub fn answer_1(input: &str) -> i32 {
    let _instructions = parse_input(input);
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
