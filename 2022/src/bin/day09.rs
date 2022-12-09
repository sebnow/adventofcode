use anyhow::{anyhow, Result};
use std::collections::HashSet;

use aocutil::{Point, Vector};

const MAX_DISTANCE: i64 = 2;

type Rope = (Point, Point);

fn step_rope((mut head, tail): Rope, step: Vector) -> Rope {
    head += step;

    (
        head,
        tail + match (head.x - tail.x, head.y - tail.y) {
            (dx, dy) if dx.abs() >= MAX_DISTANCE || dy.abs() >= MAX_DISTANCE => {
                Vector::new(dx.signum(), dy.signum())
            }
            _ => Vector::zero(),
        },
    )
}

fn move_rope(mut rope: Rope, m: Vector) -> Vec<Rope> {
    let mut history = Vec::new();
    let step = Vector::new(m.x.signum(), m.y.signum());

    let target = rope.0 + m;
    while rope.0 != target {
        history.push(rope);
        rope = step_rope(rope, step);
    }

    history.push(rope);
    history
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

    input
        .into_iter()
        .fold(
            (HashSet::<Point>::new(), (Point::zero(), Point::zero())),
            |(mut visited, rope), m| {
                let ropes = move_rope(rope, m);
                visited.extend(ropes.iter().map(|r| r.1));
                (visited, ropes[ropes.len() - 1])
            },
        )
        .0
        .len()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day09.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(day09_example_1, part_one, 9, 1, 1);
    test_example!(day09_example_2, part_two, 9, 2, 1);
}
