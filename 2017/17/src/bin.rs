extern crate adventofcode201717;

use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let n = input.trim().parse().unwrap();
    println!("Part 1: {}", adventofcode201717::answer_1(n));
    println!("Part 2: {}", adventofcode201717::answer_2(n));
}
