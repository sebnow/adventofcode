use std::{cmp::Ordering, str::FromStr, unreachable};

use anyhow::Result;
use itertools::Itertools;

const STRENGTH: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Card(char);

impl Card {
    fn strength(&self) -> usize {
        STRENGTH.iter().position(|&c| c == self.0).unwrap()
    }
}

struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut cards: [Card; 5] = [Card('a'); 5];

        for (idx, c) in s.chars().enumerate() {
            let card = Card(c);
            cards[idx] = card;
        }

        Ok(Hand { cards })
    }
}

impl Hand {
    fn kind(&self) -> usize {
        let counts = self.cards.into_iter().counts();
        let counts = counts.values().sorted().rev().collect_vec();

        match counts.as_slice() {
            [5] => 7,
            [4, 1] => 6,
            [3, 2] => 5,
            [3, 1, 1] => 4,
            [2, 2, 1] => 3,
            [2, 1, 1, 1] => 2,
            [1, 1, 1, 1, 1] => 1,
            _ => unreachable!(),
        }
    }
}

fn parse_input(s: &str) -> Vec<(Hand, u64)> {
    s.lines()
        .map(|l| {
            let (h, b) = l.split_once(' ').unwrap();
            (h.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let mut input = parse_input(s);
    input.sort_by(|a, b| match a.0.kind().cmp(&b.0.kind()) {
        Ordering::Equal => {
            a.0.cards
                .iter()
                .zip(b.0.cards.iter())
                .map(|(a, b)| a.strength().cmp(&b.strength()))
                .find(|o| !matches!(o, Ordering::Equal))
                .unwrap()
        }
        o => o,
    });

    input
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u64 * bid)
        .sum::<u64>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);

    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day07.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day07 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 7, 1, 1);
    test_example!(example_2_1, part_two, 7, 2, 1);
}
