use anyhow::Context;
use std::collections::{HashSet, VecDeque};

use aocutil::{Point, MASK_CROSSHAIR};
use itertools::Itertools;

type Grid = aocutil::Grid<Digit>;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Digit(u32);

impl std::convert::TryFrom<char> for Digit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Digit(value.to_digit(10).context("invalid digit")?))
    }
}

fn parse_input(s: &str) -> Grid {
    s.parse().unwrap()
}

fn part_one(s: &str) -> String {
    let grid = parse_input(s);

    let output: u32 = grid
        .iter()
        .filter_map(|(&p, &x)| {
            if grid.surrounding(&p, MASK_CROSSHAIR).all(|(_, &y)| x < y) {
                Some(x.0 + 1)
            } else {
                None
            }
        })
        .sum();

    format!("{}", output)
}

#[derive(Default)]
struct Basin {
    visited: HashSet<Point>,
    queue: VecDeque<Point>,
}

impl Basin {
    fn get_size(&mut self, grid: &Grid) -> usize {
        while let Some(p) = self.queue.pop_front() {
            if self.visited.contains(&p) {
                continue;
            }

            self.queue
                .extend(grid.surrounding(&p, MASK_CROSSHAIR).filter_map(|(p, &x)| {
                    if x.0 < 9 {
                        Some(p)
                    } else {
                        None
                    }
                }));

            self.visited.insert(p);
        }

        self.visited.len()
    }
}
fn part_two(s: &str) -> String {
    let grid = parse_input(s);

    // find surrounding recurrently
    // count all unique points
    let basins: usize = grid
        .iter()
        .filter(|(&p, &x)| grid.surrounding(&p, MASK_CROSSHAIR).all(|(_, &y)| x < y))
        .map(|(&p, _)| {
            let mut b = Basin::default();
            b.queue.push_back(p);
            b.get_size(&grid)
        })
        .sorted()
        .rev()
        .take(3)
        .product();

    format!("{}", basins)
}

fn main() {
    let input = include_str!("../../../../input/2021/day09.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_9_1, part_one, 9, 1, 1);
    test_example!(example_9_2, part_two, 9, 2, 1);
}
