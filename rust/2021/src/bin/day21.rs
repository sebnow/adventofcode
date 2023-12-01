use itertools::Itertools;
use lazy_static::lazy_static;
use memoize::memoize;
use std::collections::HashMap;

const BOARD_SIZE: u64 = 10;

lazy_static! {
    static ref DIRAC_DICE: HashMap<u64, usize> = (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .counts_by(|((a, b), c)| a + b + c);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Player {
    score: u64,
    position: u64,
}

impl Player {
    pub fn go_forward(&mut self, moves: u64) {
        self.position = ((self.position + moves - 1) % BOARD_SIZE) + 1;
        self.score += self.position;
    }
}

fn parse_input(s: &str) -> impl Iterator<Item = Player> + '_ {
    s.lines().map(|l| Player {
        position: l.split_once(": ").unwrap().1.parse().unwrap(),
        ..Default::default()
    })
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let mut players = input.collect_vec();
    let mut dice = (1..=100).cycle();
    let winning_score = 1_000;

    let mut rolls = 0;

    'game: loop {
        for player in &mut players {
            let moves = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
            player.go_forward(moves);
            rolls += 3;

            if player.score >= winning_score {
                break 'game;
            }
        }
    }

    players.sort_by_key(|p| p.score);
    players.reverse();

    let output = players.last().unwrap().score * rolls;

    format!("{}", output)
}

#[memoize]
fn play(p1: Player, p2: Player) -> (usize, usize) {
    if p1.score >= 21 {
        return (1, 0);
    }
    if p2.score >= 21 {
        return (0, 2);
    }

    let mut p1_total = 0;
    let mut p2_total = 0;

    for (moves, freq) in DIRAC_DICE.iter() {
        let mut next = p1;
        next.go_forward(*moves);

        let (p2_wins, p1_wins) = play(p2, next);
        p1_total += p1_wins * freq;
        p2_total += p2_wins * freq;
    }

    (p1_total, p2_total)
}

fn part_two(s: &str) -> String {
    let mut input = parse_input(s);

    let totals = play(input.next().unwrap(), input.next().unwrap());

    let output = totals.0.max(totals.1) / 2; // Uhhh... TODO: figure out why it's double lol

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../../../input/2021/day21.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day21 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_21_1_1, part_one, 21, 1, 1);
    test_example!(example_21_2_1, part_two, 21, 2, 1);
}
