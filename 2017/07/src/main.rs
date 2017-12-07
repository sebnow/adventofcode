use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn answer1<R: BufRead>(reader: R) -> String {
    String::from("")
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part_1() {
        let file = File::open("example1.txt").unwrap();
        let reader = BufReader::new(file);

        assert_eq!(answer1(reader), String::from("tknk"))
    }
}
