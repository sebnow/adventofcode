use std::fs::File;
use std::io::{BufRead, BufReader};

fn answer_1<R: BufRead>(reader: R) -> i32 {
    reader.lines()
        .map(|l| {
            let xs: Vec<i32> = l.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            xs.iter().max().unwrap() - xs.iter().min().unwrap()
        })
        .sum()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    println!("Part 1: {:?}", answer_1(reader));
    println!("Part 2: ?");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer_1() {
        assert_eq!(answer_1(Cursor::new("5 1 9 5\n7 5 3\n2 4 6 8")), 18);
    }
}
