use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;

        write!(
            f,
            "{}",
            match self {
                Up => "U",
                Right => "R",
                Down => "D",
                Left => "L",
            }
        )
    }
}

#[derive(Debug)]
pub struct Vector {
    dir: Direction,
    distance: i64,
}

#[derive(Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<Vector> for Point {
    fn from(v: Vector) -> Self {
        match v.dir {
            Direction::Up => Point {
                x: 0,
                y: v.distance,
            },
            Direction::Right => Point {
                x: v.distance,
                y: 0,
            },
            Direction::Down => Point {
                x: 0,
                y: -v.distance,
            },
            Direction::Left => Point {
                x: -v.distance,
                y: 0,
            },
        }
    }
}

impl std::str::FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, l) = s.split_at(1);

        let dir = match d {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(anyhow!("invalid direction: {}", d)),
        }?;

        let distance: i64 = l.parse()?;

        Ok(Vector { dir, distance })
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.dir, self.distance)
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vector> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .collect()
}

#[aoc(day3, part1)]
fn answer_1(input: &[Vector]) -> i64 {
    let vectors_a = input[0];
    let vectors_b = input[1];

    let paths = HashMap::new();

    let mut point = Point{x: 0, y: 0};
    for v in vectors_a {
        point += v.into();

        paths.insert(poingg

    }

    println!("{:?}", input);
    0
}

#[aoc(day3, part2)]
fn answer_2(input: &[Vector]) -> i64 {
    0
}
