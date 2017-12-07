extern crate regex;
mod tower;

use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;
use tower::{Program, Tower};

fn parse_leafs(input: &str) -> Vec<String> {
    input.split(", ").map(|x| x.to_owned()).collect()
}

fn track_from_input<R: BufRead>(tower: &mut Tower, reader: R) {
    let mut links: Vec<(String, String)> = vec![];
    let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> ((?:\w+)(?:, (?:\w+))*))?$").unwrap();

    for l in reader.lines().map(|l| l.unwrap()) {
        let caps = re.captures(&l).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let weight: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let leafs = caps.get(3).map_or(vec![], |x| parse_leafs(x.as_str()));

        tower.add(Program::new(name.to_owned(), weight));
        for leaf in leafs {
            links.push((name.to_owned(), leaf));
        }
    }

    for (parent, child) in links {
        tower.link(&parent, &child)
    }
}

fn main() {
    let mut tower = Tower::new();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    track_from_input(&mut tower, reader);

    println!("Answer 1: {:?}", tower.root().unwrap().name);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;
    use std::io::Cursor;
    use std::fs::File;

    #[test]
    fn test_example_part_1() {
        let mut tower = Tower::new();
        let file = File::open("example1.txt").unwrap();
        let reader = BufReader::new(file);
        track_from_input(&mut tower, reader);

        assert_eq!(tower.root().unwrap().name, String::from("tknk"))
    }
}
