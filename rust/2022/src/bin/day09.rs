use anyhow::{anyhow, Result};
use std::collections::HashSet;

use aocutil::{Point, Vector};

const MAX_DISTANCE: i64 = 2;

type Rope = Vec<Point>;

fn simulate(length: usize, motions: &[Vector]) -> usize {
    let mut visited = HashSet::new();
    let mut rope: Rope = Vec::with_capacity(length);
    rope.resize_with(length, Default::default);

    for motion in motions {
        let target = rope[0] + *motion;

        let step = Vector::new(motion.x.signum(), motion.y.signum());
        while rope[0] != target {
            rope[0] += step;

            for idx in 1..rope.len() {
                let (head, tail) = (rope[idx - 1], rope[idx]);
                rope[idx] += match (head.x - tail.x, head.y - tail.y) {
                    (dx, dy) if dx.abs() < MAX_DISTANCE && dy.abs() < MAX_DISTANCE => {
                        Vector::zero()
                    }
                    (dx, dy) => Vector::new(dx.signum(), dy.signum()),
                }
            }

            visited.insert(rope[rope.len() - 1]);
        }
    }

    visited.len()
}

fn parse_input(s: &str) -> Result<Vec<Vector>> {
    s.lines()
        .flat_map(|l| {
            l.split_once(' ').map(|(m, d)| {
                Ok(match m {
                    "R" => Vector::new(d.parse()?, 0),
                    "L" => Vector::new(-d.parse::<i64>()?, 0),
                    "U" => Vector::new(0, d.parse()?),
                    "D" => Vector::new(0, -d.parse::<i64>()?),
                    _ => return Err(anyhow!("invalid motion {}", m)),
                })
            })
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    simulate(2, &input).to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    simulate(10, &input).to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2022/day09.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(day09_1, part_one, 9, 1, 1);
    test_example!(day09_2, part_two, 9, 2, 1);
}
