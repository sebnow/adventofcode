extern crate adventofcode201716;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", adventofcode201716::answer_1(&input));
    println!("Part 2: {}", adventofcode201716::answer_2(&input));
}
