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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "░",
                Tile::Block => "█",
                Tile::HorizontalPaddle => "▂",
                Tile::Ball => "●",
            }
        )
    }
}

struct Game {
    score: i64,
    paddle: Point,
    ball: Point,
    grid: HashMap<Point, Tile>,
}

enum Input {
    JoystickLeft,
    JoystickNeutral,
    JoystickRight,
}

impl From<Input> for i64 {
    fn from(i: Input) -> Self {
        match i {
            Input::JoystickLeft => -1,
            Input::JoystickNeutral => 0,
            Input::JoystickRight => 1,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            grid: HashMap::new(),
            paddle: Point::default(),
            ball: Point::default(),
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
        intcode::State::AwaitingInput => Err(anyhow!("expected input")),
    }
}

fn run_game(input: &[i64]) -> Result<Game> {
    let mut prg = intcode::Interpretor::new(input);
    let mut game = Game::new();
    let debug: bool = std::env::var("DEBUG")
        .map(|x| x.parse().unwrap_or(false))
        .unwrap_or(false);

    loop {
        match prg.run()? {
            intcode::State::Terminated(_) => return Ok(game),
            intcode::State::Suspended(x) => {
                let y = get_output(&mut prg)?;
                let v = get_output(&mut prg)?;
                if x == -1 && y == 0 {
                    game.score = v;
                } else {
                    let point = Point::new(x, 0 - y);
                    let tile = Tile::try_from(v)?;
                    game.grid.insert(point, tile);
                    match tile {
                        Tile::Ball => game.ball = point,
                        Tile::HorizontalPaddle => game.paddle = point,
                        _ => (),
                    }
                }

                if debug {
                    println!();
                    println!("{}", aocutil::Grid::from(&game.grid));
                }
            }
            intcode::State::AwaitingInput => {
                let input = match game.paddle.x.cmp(&game.ball.x) {
                    std::cmp::Ordering::Less => Input::JoystickRight,
                    std::cmp::Ordering::Greater => Input::JoystickLeft,
                    _ => Input::JoystickNeutral,
                }
                .into();

                game.paddle = game.paddle + Point::new(input, 0);
                prg.input(input);
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
fn answer_2(input: &[i64]) -> Result<i64> {
    let mut input = input.to_owned();
    input[0] = 2;
    let game = run_game(&input)?;
    Ok(game.score)
}
