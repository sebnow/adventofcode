use crate::geo::{Direction, Point};
use crate::intcode::{Interpretor, State};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

const BLACK: i64 = 0;
const WHITE: i64 = 1;

const LEFT: i64 = 0;
const RIGHT: i64 = 1;

struct Robot {
    brain: Interpretor,
    direction: Direction,
    position: Point,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day11, part1)]
fn answer_1(input: &[i64]) -> Result<usize> {
    let mut panels: HashMap<Point, i64> = HashMap::new();
    let mut painted: HashMap<Point, usize> = HashMap::new();
    let mut rbt = Robot {
        brain: Interpretor::new(input),
        direction: Direction::Up,
        position: Point::new(0, 0),
    };

    loop {
        let pos = rbt.position;
        let color = panels.get(&pos).unwrap_or(&BLACK);
        rbt.brain.input(*color);

        match rbt.brain.run()? {
            State::Suspended(color) => {
                panels.insert(pos, color);
                *painted.entry(pos).or_insert(0) += 1;
            }
            State::Terminated(_) => break,
        }

        match rbt.brain.run()? {
            State::Suspended(direction) => {
                rbt.direction = match direction {
                    LEFT => match rbt.direction {
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Right => Direction::Up,
                    },
                    RIGHT => match rbt.direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    },
                    _ => return Err(anyhow!("invalid direction {}", direction)),
                }
            }
            State::Terminated(_) => return Err(anyhow!("expected second input")),
        }

        rbt.position = pos + rbt.direction.to_point();
    }

    Ok(painted.len())
}

#[aoc(day11, part2)]
fn answer_2(input: &[i64]) -> usize {
    0
}
