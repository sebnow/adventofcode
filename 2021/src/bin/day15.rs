use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

const SURROUNDING: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

type Risk = u32;
type Point = (i64, i64);
type Grid = HashMap<Point, Risk>;

fn find_lowest_risk(grid: &Grid) -> u32 {
    let mut cell_risk = HashMap::new();

    let mut queue = BinaryHeap::from([(Reverse(0), (0, 0))]);
    while let Some((Reverse(total_risk), p @ (x, y))) = queue.pop() {
        let risk = cell_risk.entry(p).or_insert(u32::MAX);

        if total_risk < *risk {
            *risk = total_risk;

            for (dx, dy) in SURROUNDING {
                let sp = (x + dx, y + dy);

                if let Some(risk) = grid.get(&sp) {
                    queue.push((Reverse(total_risk + risk), sp));
                }
            }
        }
    }

    let bottom_right = cell_risk.keys().max().unwrap();
    cell_risk[bottom_right]
}
fn parse_input(s: &str) -> Grid {
    s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64), c.to_digit(10).unwrap()))
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let grid = parse_input(s);
    let output = find_lowest_risk(&grid);

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let grid = parse_input(s);
    let (width, height) = grid.keys().max().map(|(x, y)| (x + 1, y + 1)).unwrap();
    let grid = grid
        .into_iter()
        .flat_map(|((x, y), r)| {
            (0..5).cartesian_product(0..5).map(move |(mx, my)| {
                (
                    (x + mx * width, y + my * height),
                    (r + mx as Risk + my as Risk - 1) % 9 + 1,
                )
            })
        })
        .collect();

    let output = find_lowest_risk(&grid);

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day15.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day15 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_15_1_1, part_one, 15, 1, 1);
    test_example!(example_15_2_1, part_two, 15, 2, 1);
}
