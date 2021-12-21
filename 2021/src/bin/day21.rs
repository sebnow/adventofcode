use std::collections::HashMap;

use itertools::Itertools;

const BOARD_SIZE: u64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Player {
    score: u64,
    position: u64,
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
            player.position = ((player.position + moves - 1) % BOARD_SIZE) + 1;
            player.score += player.position;
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

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let players = input.collect_vec();
    let dice_permutations: HashMap<u64, usize> = (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .counts_by(|((a, b), c)| a + b + c);

    let output = 0;

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day21.txt");
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
