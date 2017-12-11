use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input<R: BufRead>(reader: R) -> Vec<i32> {
    vec![]
}

fn answer_1(input: &[i32]) -> i32 {
    0
}

fn answer_2(input: &[i32]) -> i32 {
    0
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let xs = parse_input(reader);

    println!("Part 1: {:?}", answer_1(&xs));
    println!("Part 2: {:?}", answer_2(&xs));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer_1() {
        assert_eq!(answer_1(&parse_input(Cursor::new("1122"))), 3);
        assert_eq!(answer_1(&parse_input(Cursor::new("1111"))), 4);
        assert_eq!(answer_1(&parse_input(Cursor::new("1234"))), 0);
        assert_eq!(answer_1(&parse_input(Cursor::new("91212129"))), 9);
    }
}
