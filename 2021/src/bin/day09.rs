use std::collections::{HashSet, VecDeque};

use aocutil::{Point, MASK_CROSSHAIR};
use itertools::Itertools;

type Grid = aocutil::Grid<u32>;

fn parse_input(s: &str) -> Grid {
    let v: Vec<Vec<_>> = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    Grid::from_vec2d(v)
}

fn part_one(s: &str) -> String {
    let grid = parse_input(s);

    let output: u32 = grid
        .iter()
        .filter_map(|(&p, &x)| {
            let around = grid.surrounding(&p, MASK_CROSSHAIR);

            if around.iter().all(|(_, &y)| x < y) {
                Some(x + 1)
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

            self.queue.extend(
                grid.surrounding(&p, MASK_CROSSHAIR)
                    .iter()
                    .filter_map(|(p, &x)| if x < 9 { Some(p) } else { None }),
            );

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
        .filter(|(&p, &x)| {
            let around = grid.surrounding(&p, MASK_CROSSHAIR);
            around.iter().all(|(_, &y)| x < y)
        })
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
    let input = include_str!("../../input/day09.txt");
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
