use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

type Score = u32;

const LOSS: Score = 0;
const DRAW: Score = 3;
const WIN: Score = 6;

enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    pub fn value(&self) -> Score {
        use Action::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum GuideMove {
    A,
    B,
    C,
}

impl FromStr for GuideMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => GuideMove::A,
            "B" => GuideMove::B,
            "C" => GuideMove::C,
            _ => return Err(anyhow!("invalid move")),
        })
    }
}

enum GuideResponse {
    X,
    Y,
    Z,
}

impl FromStr for GuideResponse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => GuideResponse::X,
            "Y" => GuideResponse::Y,
            "Z" => GuideResponse::Z,
            _ => return Err(anyhow!("invalid response")),
        })
    }
}

fn score(p1: &Action, p2: &Action) -> Score {
    use Action::*;

    p1.value()
        + match (p1, p2) {
            (Rock, Rock) => DRAW,
            (Rock, Paper) => LOSS,
            (Rock, Scissors) => WIN,
            (Paper, Rock) => WIN,
            (Paper, Paper) => DRAW,
            (Paper, Scissors) => LOSS,
            (Scissors, Rock) => LOSS,
            (Scissors, Paper) => WIN,
            (Scissors, Scissors) => DRAW,
        }
}

fn parse_input(s: &str) -> Result<Vec<(GuideMove, GuideResponse)>> {
    s.lines()
        .map(|l| {
            let mut parts = l.split(' ');
            Ok((
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing move"))?
                    .parse()?,
                parts
                    .next()
                    .ok_or_else(|| anyhow!("missing response"))?
                    .parse()?,
            ))
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();

    let map_to_actions = |round: &(GuideMove, GuideResponse)| -> (Action, Action) {
        (
            match round.0 {
                GuideMove::A => Action::Rock,
                GuideMove::B => Action::Paper,
                GuideMove::C => Action::Scissors,
            },
            match round.1 {
                GuideResponse::X => Action::Rock,
                GuideResponse::Y => Action::Paper,
                GuideResponse::Z => Action::Scissors,
            },
        )
    };

    input
        .iter()
        .map(|round| {
            let actions = map_to_actions(round);
            score(&actions.1, &actions.0)
        })
        .sum::<Score>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();

    let map_to_actions = |round: &(GuideMove, GuideResponse)| -> (Action, Action) {
        match round {
                (GuideMove::A, GuideResponse::X) => (Action::Rock, Action::Scissors),
                (GuideMove::B, GuideResponse::X) => (Action::Paper, Action::Rock),
                (GuideMove::C, GuideResponse::X) => (Action::Scissors, Action::Paper),
                (GuideMove::A, GuideResponse::Y) => (Action::Rock, Action::Rock),
                (GuideMove::B, GuideResponse::Y) => (Action::Paper, Action::Paper),
                (GuideMove::C, GuideResponse::Y) => (Action::Scissors, Action::Scissors),
                (GuideMove::A, GuideResponse::Z) => (Action::Rock, Action::Paper),
                (GuideMove::B, GuideResponse::Z) => (Action::Paper, Action::Scissors),
                (GuideMove::C, GuideResponse::Z) => (Action::Scissors, Action::Rock),
        }
    };

    input
        .iter()
        .map(|round| {
            let actions = map_to_actions(round);
            score(&actions.1, &actions.0)
        })
        .sum::<Score>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2022/day02.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_2_1, part_one, 2, 1, 1);
    test_example!(example_2_2, part_two, 2, 2, 1);
}
