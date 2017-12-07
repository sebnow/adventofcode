extern crate regex;

use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Program {
    name: String,
    weight: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct InputLine(Program, Vec<String>);

fn parse_leafs(input: &str) -> Vec<String> {
    input.split(", ").map(|x| x.to_owned()).collect()
}

fn parse_input<R: BufRead>(reader: R) -> Vec<InputLine> {
    let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> ((?:\w+)(?:, (?:\w+))*))?$").unwrap();
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let caps = re.captures(&l).unwrap();
            let name = caps.get(1).unwrap().as_str();
            let weight: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let leafs = caps.get(3).map_or(vec![], |x| parse_leafs(x.as_str()));

            InputLine(
                Program {
                    name: name.to_owned(),
                    weight: weight,
                },
                leafs,
            )
        })
        .collect()
}

fn answer1<R: BufRead>(reader: R) -> String {
    let inputs = parse_input(reader);
    String::from("")
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;
    use std::io::Cursor;
    use std::fs::File;

    #[test]
    fn test_example_part_1() {
        let file = File::open("example1.txt").unwrap();
        let reader = BufReader::new(file);

        assert_eq!(answer1(reader), String::from("tknk"))
    }

    fn test_parse_input() {
        let reader = Cursor::new("dsad (10) -> ewq, tre, vcx");

        assert_eq!(
            parse_input(reader),
            vec![
                InputLine(
                    Program {
                        name: String::from("dsad"),
                        weight: 10,
                    },
                    vec![
                        String::from("ewq"),
                        String::from("tre"),
                        String::from("vcx"),
                    ]
                ),
            ]
        );
    }
}
