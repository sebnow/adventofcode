#[macro_use]
extern crate failure;

use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn step(&self, dir: &Dir) -> Point {
        match dir {
            &Dir::Up => Point{x: self.x, y: self.y + 1},
            &Dir::Left => Point{x: self.x - 1, y: self.y},
            &Dir::Down => Point{x: self.x, y: self.y - 1},
            &Dir::Right => Point{x: self.x + 1, y: self.y},
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    pub fn left(&self) -> Dir {
        match self {
            &Dir::Up => Dir::Left,
            &Dir::Left => Dir::Down,
            &Dir::Down => Dir::Right,
            &Dir::Right => Dir::Up,
        }
    }

    pub fn right(&self) -> Dir {
        match self {
            &Dir::Up => Dir::Right,
            &Dir::Right => Dir::Down,
            &Dir::Down => Dir::Left,
            &Dir::Left => Dir::Up,
        }
    }
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut points = Vec::new();
    let mut max_y = 0;
    let mut max_x = 0;

    for (y, l) in input.lines().enumerate() {
        max_y = max(max_y, y);
        for (x, c) in l.chars().enumerate() {
            max_x = max(max_x, x);
            if c == '#' {
                points.push(Point{x: x as i32, y: y as i32});
            }
        }
    }

    let trans_y = max_y/2;
    let trans_x = max_x/2;
    for ref mut p in points.iter_mut() {
        p.x -= trans_x as i32;
        p.y -= trans_y as i32;
    }

    points.into_iter().collect()
}

pub fn answer_1(input: &str) -> u32 {
    let mut dir = Dir::Up;
    let mut map = parse_input(input);
    let mut carrier = Point{x: 0, y: 0};
    let mut infected = 0;

    println!("{:?}", map);
    for i in 0..10002 {
        if map.contains(&carrier) {
            dir = dir.right();
            map.remove(&carrier);
        } else {
            dir = dir.left();
            map.insert(carrier);
            infected += 1;
        }

        carrier = carrier.step(&dir);
    }

    infected
}

pub fn answer_2(input: &str) -> Result<i64, failure::Error> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = [
            ".#.",
            "#..",
            "..#",
        ].join("\n");

        assert_eq!(parse_input(&input), vec![
            Point{x: 0, y: -1},
            Point{x: -1, y: 0},
            Point{x: 1, y: 1}
        ].into_iter().collect());
    }

    #[test]
    fn example_1() {
        let input = [
            "..#",
            "#..",
            "...",
        ].join("\n");

        assert_eq!(5587, answer_1(&input));
    }
}
