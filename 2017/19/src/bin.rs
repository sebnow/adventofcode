extern crate adventofcode201719;

use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", adventofcode201719::answer_1(&input).unwrap());
    println!("Part 2: {}", adventofcode201719::answer_2(&input).unwrap());
}
