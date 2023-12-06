use std::collections::VecDeque;

use itertools::Itertools;

enum Error {
    Incomplete(VecDeque<char>),
    Corrupted(char),
}

const OPENING: [char; 4] = ['(', '[', '{', '<'];
const CLOSING: [char; 4] = [')', ']', '}', '>'];
const SCORE_1: [u128; 4] = [3, 57, 1197, 25137];

fn find_error<I: Iterator<Item = char>>(chars: I) -> Option<Error> {
    let mut expected: VecDeque<char> = VecDeque::default();

    for c in chars {
        if let Some(idx) = OPENING.iter().position(|&x| c == x) {
            expected.push_front(CLOSING[idx]);
            continue;
        }

        if let Some(closing) = expected.pop_front() {
            if c != closing {
                return Some(Error::Corrupted(c));
            }
        }
    }

    if expected.is_empty() {
        None
    } else {
        Some(Error::Incomplete(expected))
    }
}

fn part_one(s: &str) -> String {
    let score = |c: char| {
        CLOSING
            .iter()
            .position(|&x| x == c)
            .and_then(|idx| SCORE_1.get(idx))
    };

    let output: u128 = s
        .lines()
        .filter_map(|l| {
            find_error(l.chars()).and_then(|e| match e {
                Error::Corrupted(c) => score(c),
                _ => None,
            })
        })
        .sum();

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let score = |c: char| CLOSING.iter().position(|&x| x == c).map(|x| x as u128 + 1);

    let output: Vec<_> = s
        .lines()
        .filter_map(|l| {
            find_error(l.chars()).and_then(|e| match e {
                Error::Incomplete(r) => Some(r),
                _ => None,
            })
        })
        .map(|r| {
            r.iter()
                .fold(0, |acc, &c| (acc * 5) + score(c).unwrap_or(0))
        })
        .sorted()
        .collect();

    assert!(output.len() % 2 != 0);
    format!("{}", output[output.len() / 2])
}

fn main() {
    let input = include_str!("../../../../input/2021/day10.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_10_1, part_one, 10, 1, 1);
    test_example!(example_10_2, part_two, 10, 2, 1);
}
