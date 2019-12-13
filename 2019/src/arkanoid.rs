use crate::intcode;
use anyhow::{anyhow, Result};
use aocutil;
use std::collections::HashMap;
use std::convert::TryFrom;

type Point = aocutil::Point<i64>;

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
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

pub struct Game {
    init: bool,
    over: bool,
    score: i64,
    paddle: Point,
    ball: Point,
    prg: intcode::Interpretor,
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
    pub fn new(rom: &[i64]) -> Self {
        Game {
            init: false,
            over: false,
            score: 0,
            prg: intcode::Interpretor::new(rom),
            grid: HashMap::new(),
            paddle: Point::default(),
            ball: Point::default(),
        }
    }

    pub fn update(&mut self) -> Result<()> {
        match self.prg.run()? {
            intcode::State::Terminated(_) => {
                self.over = true;
            }
            intcode::State::Suspended(x) => {
                let y = self.get_output()?;
                let v = self.get_output()?;
                if x == -1 && y == 0 {
                    self.score = v;
                } else {
                    let point = Point::new(x, 0 - y);
                    let tile = Tile::try_from(v)?;
                    self.grid.insert(point, tile);
                    match tile {
                        Tile::Ball => self.ball = point,
                        Tile::HorizontalPaddle => self.paddle = point,
                        Tile::Empty => {
                            self.grid.remove(&point);
                        }
                        _ => (),
                    }
                }
            }
            intcode::State::AwaitingInput => {
                self.init = true;
                let input = match self.paddle.x.cmp(&self.ball.x) {
                    std::cmp::Ordering::Less => Input::JoystickRight,
                    std::cmp::Ordering::Greater => Input::JoystickLeft,
                    _ => Input::JoystickNeutral,
                }
                .into();

                self.paddle = self.paddle + Point::new(input, 0);
                self.prg.input(input);
            }
        }

        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.init
    }

    pub fn is_over(&self) -> bool {
        self.over
    }

    pub fn get_score(&self) -> i64 {
        self.score
    }

    pub fn get_tiles<'a>(&'a self) -> &'a HashMap<Point, Tile> {
        &self.grid
    }

    pub fn count_blocks(&self) -> usize {
        self.grid.iter().filter(|(_, &t)| t == Tile::Block).count()
    }

    fn get_output(&mut self) -> Result<i64> {
        match self.prg.run()? {
            intcode::State::Suspended(x) => Ok(x),
            intcode::State::Terminated(_) => Err(anyhow!("expected output")),
            intcode::State::AwaitingInput => Err(anyhow!("expected input")),
        }
    }
}
