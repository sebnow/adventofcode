use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input<R: BufRead>(reader: R) -> Vec<u8> {
    let mut input: Vec<u8> = reader
        .bytes()
        .map(|x| x.unwrap())
        .filter_map(|x| if x > 48 {Some(x - 48)} else {None})
        .collect();

    let first = input[0];
    input.push(first);

    input
}

fn answer_1(input: &[u8]) -> i32 {
    let mut dupes = Vec::new();
    let mut current = input[0];
    let mut sum: i32 = 0;

    for &x in input.iter().skip(1) {
        if x == current {
            sum += x as i32;
        } else {
            dupes.push(sum);
            sum = 0;
            current = x;
        }
    }

    if sum > 0 {
        dupes.push(sum);
    }

    dupes.iter().sum()
}

fn answer_2(input: &[u8]) -> i32 {
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
