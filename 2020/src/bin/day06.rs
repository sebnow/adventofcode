extern crate itertools;
use std::collections::HashMap;
use itertools::Itertools;

fn part_one(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| group.lines().flat_map(|l| l.chars()).unique().count())
        .sum::<usize>()
        .to_string()
}

fn part_two(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group|{
            let mut all: HashMap::<char, usize> = HashMap::default();
            let people = group.lines().count();

            for c in group.lines().flat_map(|l| l.chars()) {
                let entry = all.entry(c).or_insert(0);
                *entry += 1;
            }

            all.iter().filter(|(_, &count)| count == people).count()
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("../../input/day06.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 6, 1, 1);
    test_example!(example_two_2, part_two, 6, 2, 1);
}
