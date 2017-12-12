use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input<R: BufRead>(reader: R) -> () {
    ()
}

fn answer_1(input: &()) -> i32 {
    0
}

fn answer_2(input: &()) -> i32 {
    0
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let input = parse_input(reader);

    println!("Part 1: {:?}", answer_1(&input));
    println!("Part 2: {:?}", answer_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer() {
        let input = Cursor::new(
            "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5",
        );
        assert_eq!(answer_1(&parse_input(input)), 6);
    }
}
