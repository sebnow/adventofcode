use crate::intcode;
use anyhow::{anyhow, Result};
use aocutil;
use std::collections::HashMap;
use std::convert::TryFrom;

type Point = aocutil::Point<i64>;

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

struct Game {
    score: i64,
    grid: HashMap<Point, Tile>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            grid: HashMap::new(),
        }
    }
}

impl TryFrom<i64> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use Tile::*;
        match value {
            0 => Ok(Empty),
            1 => Ok(Wall),
            2 => Ok(Block),
            3 => Ok(HorizontalPaddle),
            4 => Ok(Ball),
            _ => Err(anyhow!("invalid tile {}")),
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

fn get_output(prg: &mut intcode::Interpretor) -> Result<i64> {
    match prg.run()? {
        intcode::State::Suspended(x) => Ok(x),
        intcode::State::Terminated(_) => Err(anyhow!("expected output")),
    }
}

fn run_game(input: &[i64]) -> Result<Game> {
    let mut prg = intcode::Interpretor::new(input);
    let mut game = Game::new();

    loop {
        match prg.run()? {
            intcode::State::Terminated(_) => return Ok(game),
            intcode::State::Suspended(x) => {
                let y = get_output(&mut prg)?;
                let v = get_output(&mut prg)?;
                if x == -1 && y == 0 {
                    game.score = v;
                } else {
                    let point = Point::new(x, y);
                    let tile = Tile::try_from(v)?;
                    game.grid.insert(point, tile);
                }
            }
        }
    }
}

#[aoc(day13, part1)]
fn answer_1(input: &[i64]) -> Result<usize> {
    let game = run_game(input)?;
    Ok(game.grid.iter().filter(|(_, &t)| t == Tile::Block).count())
}

#[aoc(day13, part2)]
fn answer_2(_input: &[i64]) -> Result<usize> {
    Ok(0)
}
