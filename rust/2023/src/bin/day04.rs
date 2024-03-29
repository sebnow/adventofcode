use std::{collections::VecDeque, iter::repeat};

use anyhow::Result;

#[derive(Debug)]
struct Card {
    winning: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    pub fn matches(&self) -> usize {
        self.winning
            .iter()
            .filter(|w| self.numbers.contains(w))
            .count()
    }
}

fn parse_nums(s: &str) -> Vec<u8> {
    s.split(' ')
        .filter_map(|n| {
            if n.is_empty() {
                None
            } else {
                Some(n.trim_start().parse().expect("invalid winning number"))
            }
        })
        .collect()
}

fn parse_input(s: &str) -> Vec<Card> {
    s.lines()
        .map(|l| {
            let (_, right) = l.split_once(": ").expect("invalid card");
            let (win_side, num_side) = right.split_once(" | ").expect("invalid numbers");

            Card {
                winning: parse_nums(win_side),
                numbers: parse_nums(num_side),
            }
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);

    input
        .iter()
        .map(|card| {
            let matches = card.matches() as u32;
            if matches == 0 {
                0
            } else {
                2_u64.pow(matches - 1)
            }
        })
        .sum::<u64>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let mut counts: Vec<usize> = repeat(1).take(input.len()).collect();

    for (idx, card) in input.iter().enumerate() {
        for copied in idx + 1..=idx + card.matches() {
            counts[copied] += counts[idx];
        }
    }

    counts.iter().sum::<usize>().to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day04.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day04 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 4, 1, 1);
    test_example!(example_2_1, part_two, 4, 2, 1);
}
