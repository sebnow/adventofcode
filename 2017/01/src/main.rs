use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input<R: BufRead>(reader: R) -> Vec<i32> {
    reader
        .bytes()
        .map(|x| x.unwrap())
        .filter_map(|x| if x >= b'0' {Some((x - b'0') as i32)} else {None})
        .collect()
}

fn sum_dupe(xs: &[i32], skip: usize) -> i32 {
    let lookahead = xs.iter().cycle().skip(skip);
    xs.iter()
        .zip(lookahead)
        .filter_map(|(a, b)| match a == b {
            true => Some(a),
            false => None,
        })
        .sum()
}

fn answer_1(input: &[i32]) -> i32 {
    sum_dupe(input, 1)
}

fn answer_2(input: &[i32]) -> i32 {
    sum_dupe(input, input.len() / 2)
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

    #[test]
    fn example_answer_2() {
        assert_eq!(answer_2(&parse_input(Cursor::new("1212"))), 6);
        assert_eq!(answer_2(&parse_input(Cursor::new("1221"))), 0);
        assert_eq!(answer_2(&parse_input(Cursor::new("123425"))), 4);
        assert_eq!(answer_2(&parse_input(Cursor::new("123123"))), 12);
        assert_eq!(answer_2(&parse_input(Cursor::new("12131415"))), 4);
    }
}
