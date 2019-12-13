use crate::arkanoid::Game;
use anyhow::Result;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day13, part1)]
fn answer_1(input: &[i64]) -> Result<usize> {
    let mut game = Game::new(input);
    while !game.is_over() {
        game.update();
    }

    Ok(game.count_blocks())
}

#[aoc(day13, part2)]
fn answer_2(input: &[i64]) -> Result<i64> {
    let mut input = input.to_owned();
    input[0] = 2;

    let mut game = Game::new(&input);
    while !game.is_over() {
        game.update();
    }

    Ok(game.get_score())
}
