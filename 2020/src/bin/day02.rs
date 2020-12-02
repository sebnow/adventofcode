use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Policy {
    min: usize,
    max: usize,
    ch: char,
}

impl FromStr for Policy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matches = s.split(" ");
        let minmax = matches.next().ok_or_else(|| anyhow!("missing min/max"))?;
        let ch = matches
            .next()
            .ok_or_else(|| anyhow!("missing policy character"))?
            .chars()
            .next()
            .ok_or_else(|| anyhow!("password character not parsed"))?;

        let mut matches = minmax.split("-");
        let min = matches
            .next()
            .ok_or_else(|| anyhow!("missing min"))?
            .parse()?;
        let max = matches
            .next()
            .ok_or_else(|| anyhow!("missing max"))?
            .parse()?;

        Ok(Policy { min, max, ch })
    }
}

#[derive(PartialEq, Debug)]
struct PasswordEntry {
    policy: Policy,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matches = s.split(": ");
        let policy = matches
            .next()
            .ok_or_else(|| anyhow!("missing policy"))?
            .parse()?;
        let password = matches
            .next()
            .ok_or_else(|| anyhow!("missing password"))?
            .to_string();

        Ok(PasswordEntry { policy, password })
    }
}

fn parse_input(s: &str) -> Result<Vec<PasswordEntry>> {
    s.lines().map(|l| l.parse()).collect()
}

fn part_one(input: &str) -> String {
    parse_input(input)
        .unwrap()
        .iter()
        .filter(|entry| {
            let count = entry
                .password
                .chars()
                .filter(|&c| c == entry.policy.ch)
                .count();

            entry.policy.min <= count && count <= entry.policy.max
        })
        .count()
        .to_string()
}

fn part_two(input: &str) -> String {
    parse_input(input)
        .unwrap()
        .iter()
        .filter(|entry| {
            let pass = entry.password.chars().collect::<Vec<_>>();
            let ch = entry.policy.ch;

            let fst = pass[entry.policy.min - 1];
            let snd = pass[entry.policy.max - 1];

            (fst == ch) ^ (snd == ch)
        })
        .count()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day02.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    #[test]
    fn parse() {
        assert_eq!(
            PasswordEntry {
                policy: Policy {
                    min: 1,
                    max: 3,
                    ch: 'a',
                },
                password: "abcde".into(),
            },
            "1-3 a: abcde".parse().unwrap()
        );
    }

    test_example!(example_one_1, part_one, 2, 1, 1);
    test_example!(example_two_1, part_two, 2, 2, 1);
}
