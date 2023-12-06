use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

fn parse_input(s: &str) -> Result<&str> {
    Ok(s)
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    input
        .lines()
        .map(|l: &str| -> u32 {
            let mut digits = l.chars().filter(|c| c.is_ascii_digit());
            let first = digits.next().expect("first digit not found");
            let last = digits.last().unwrap_or(first);
            format!("{}{}", first, last).parse().expect("parsing digit")
        })
        .sum::<u32>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let digits = [
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    input
        .lines()
        .map(|l| -> u64 {
            let mut locs = digits
                .iter()
                .flat_map(|d| l.match_indices(d.0).map(|p| (p, d.1)))
                .sorted_by(|a, b| a.0.cmp(&b.0));

            let first = locs.next().expect("missing first digit");
            let last = locs.last().unwrap_or(first);

            first.1 * 10 + last.1
        })
        .sum::<u64>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day01.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 1, 1, 1);
    test_example!(example_1_2, part_two, 1, 2, 1);
}
