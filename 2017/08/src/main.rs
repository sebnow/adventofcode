use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn process(input: &str) -> HashMap<&str, i32> {
    HashMap::new()
}

fn answer_1(input: &str) -> i32 {
    *process(input).values().max().unwrap_or(&0)
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {:?}", answer_1(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";
        assert_eq!(answer_1(input), 1);
    }

}
