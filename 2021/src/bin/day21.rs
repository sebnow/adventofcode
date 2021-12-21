use itertools::Itertools;

const BOARD_SIZE: u64 = 10;
const WINNING_SCORE: u64 = 1_000;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Player {
    score: u64,
    position: u64,
}

fn play<DI: Iterator<Item = u64>>(
    mut dice: DI,
    mut players: Vec<Player>,
) -> (Vec<Player>, u64) {
    let mut rolls = 0;

    loop {
        for player in &mut players {
            let moves = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
            let last_pos = player.position;
            player.position = ((player.position + moves - 1) % BOARD_SIZE) + 1;
            player.score += player.position;
            rolls += 3;

            if player.score >= WINNING_SCORE{
                return (players.iter().sorted_by_key(|p| p.score).rev().copied().collect_vec(), rolls)
            }
        }
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
    let players = input.collect_vec();
    let dice = (1..=100).cycle();

    let (players, rolls) = play(dice, players);
    let output = players.last().unwrap().score * rolls;

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let players = input.collect_vec();

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
