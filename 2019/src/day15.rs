use crate::intcode;
use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};
use std::convert::{Into, TryFrom};

type Point = aocutil::Point<i64>;
type Map = HashMap<Point, Tile>;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Oxygen,
    //Robot,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => "░",
                Tile::Wall => "█",
                Tile::Oxygen => "X",
                //Tile::Robot => "O",
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Movement {
    North,
    East,
    South,
    West,
}

impl Into<i64> for Movement {
    fn into(self) -> i64 {
        match self {
            Movement::North => 1,
            Movement::East => 4,
            Movement::South => 2,
            Movement::West => 3,
        }
    }
}

impl Into<Point> for Movement {
    fn into(self) -> Point {
        match self {
            Movement::North => Point::new(0, 1),
            Movement::East => Point::new(1, 0),
            Movement::South => Point::new(0, -1),
            Movement::West => Point::new(-1, 0),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    HitWall,
    Moved,
    AtDestination,
}

impl TryFrom<i64> for Status {
    type Error = anyhow::Error;

    fn try_from(status: i64) -> Result<Self, Self::Error> {
        match status {
            0 => Ok(Status::HitWall),
            1 => Ok(Status::Moved),
            2 => Ok(Status::AtDestination),
            _ => Err(anyhow!("invalid status")),
        }
    }
}

#[derive(Clone)]
struct Robot {
    prg: intcode::Interpretor,
    pos: Point,
    steps: usize,
}

impl Robot {
    pub fn new(prg: &intcode::Interpretor) -> Self {
        Robot {
            prg: prg.clone(),
            pos: Point::new(0, 0),
            steps: 0,
        }
    }
}

fn build_map(prg: &intcode::Interpretor) -> Result<(Map, Option<Robot>)> {
    let directions = [
        Movement::North,
        Movement::East,
        Movement::West,
        Movement::South,
    ];
    let mut map = HashMap::new();
    let mut robots = VecDeque::new();
    let mut found = None;

    {
        let robot = Robot::new(prg);
        map.insert(robot.pos, Tile::Empty);
        robots.push_front(robot);
    }

    while let Some(robot) = robots.pop_front() {
        for &m in &directions {
            let new_pos = robot.pos + m.into();
            if map.contains_key(&new_pos) {
                // visited already
                continue;
            }

            let mut new_robot = robot.clone();
            new_robot.prg.input(m.into());
            match new_robot.prg.run()? {
                intcode::State::Terminated(_) => break,
                intcode::State::Suspended(out) => {
                    match Status::try_from(out)? {
                        Status::HitWall => {
                            map.insert(new_pos, Tile::Wall);
                        }
                        Status::Moved => {
                            map.insert(new_pos, Tile::Empty);
                            new_robot.pos = new_pos;
                            new_robot.steps += 1;
                            robots.push_front(new_robot);
                        }
                        Status::AtDestination => {
                            map.insert(new_pos, Tile::Oxygen);
                            new_robot.pos = new_pos;
                            new_robot.steps += 1;
                            found = Some(new_robot.clone());
                            robots.push_front(new_robot);
                        }
                    };
                }
                intcode::State::AwaitingInput => panic!("Input should have already been provided"),
            }
            //println!("\n\n{}", aocutil::Grid::from(&map));
        }
    }

    Ok((map, found))
}

fn fill(map: &Map, origin: &Point) -> Result<usize> {
    let mut map = map.clone();
    let directions = [
        Movement::North,
        Movement::East,
        Movement::West,
        Movement::South,
    ];
    let mut oxygen = Vec::new();
    let mut to_fill = VecDeque::new();
    to_fill.push_front((*origin, 0));

    while let Some((p, minutes)) = to_fill.pop_front() {
        oxygen.push((p, minutes));
        map.insert(p, Tile::Oxygen);

        for &m in &directions {
            let new_pos = p + m.into();
            match map.get(&new_pos) {
                Some(Tile::Wall) => continue,
                Some(Tile::Oxygen) => continue,
                Some(Tile::Empty) => {
                    to_fill.push_back((new_pos, minutes + 1));
                }
                None => return Err(anyhow!("something went wrong")),
            }
        }
    }

    Ok(oxygen
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(_, m)| *m)
        .unwrap())
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day15, part1)]
fn answer_1(input: &[i64]) -> Result<usize> {
    let prg = intcode::Interpretor::new(input);
    let (map, robot) = build_map(&prg)?;

    Ok(robot.unwrap().steps)
}

#[aoc(day15, part2)]
fn answer_2(input: &[i64]) -> Result<usize> {
    let prg = intcode::Interpretor::new(input);
    let (map, robot) = build_map(&prg)?;

    let minutes = fill(&map, &robot.unwrap().pos)?;
    Ok(minutes)
}
