use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

const RANGE: usize = 1000;
const BEACON_OVERLAP: usize = 12;

type Point = nalgebra::Point3<i64>;
type Matrix = nalgebra::Matrix3<i64>;
type Grid = HashMap<Point, Cell>;

enum Cell {
    Beacon,
    Scanner,
}

#[derive(Clone, Debug)]
struct Scanner {
    id: i64,
    beacons: Vec<Point>,
}

fn debug(msg: &str) {
    println!("{}", msg);
}

fn reconstruct<I: Iterator<Item = Scanner>>(scanners: I, overlap_requirement: usize) -> Grid {
    #[inline]
    fn add_scanner (grid: &mut Grid, p: &Point, s: &Scanner) {
        grid.insert(*p, Cell::Scanner);
        grid.extend(s.beacons.iter().map(|p| (*p, Cell::Beacon)));
    }

    let mut queue: VecDeque<_> = scanners.collect();
    let mut grid = Grid::new();
    let s = queue.pop_front().unwrap();
    add_scanner(&mut grid, &[0,0,0].into(), &s);

    // TODO: How is the range relevant? Just a way to optimize the search space?
    // TODO: Calculate distances between all points

    while let Some(s) = queue.pop_front() {
        // TODO: For all possible transforms
        let beacons = &s.beacons;
        // TODO: Calculate distances between all points and see if they match distances in the grid
        let overlapping = beacons.iter().filter(|b| grid.contains_key(b)).count();
        if overlapping >= overlap_requirement {
            debug(&format!("Scanner {} has overlapping beacons", s.id));
            // TODO: Apply transform to point
            add_scanner(&mut grid, &[0,0,0].into(), &s);
            continue
        }

        queue.push_back(s);
    }

    grid
}

fn parse_input(s: &str) -> impl Iterator<Item = Scanner> + '_ {
    s.split("\n\n").enumerate().map(|(id, block)| Scanner {
        id: id as i64,
        beacons: block
            .lines()
            .skip(1)
            .map(|l| {
                let mut coords = l.split(',').map(|x| x.parse().unwrap());
                [
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                ]
                .into()
            })
            .collect_vec(),
    })
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let grid = reconstruct(input, BEACON_OVERLAP);

    let output = grid.values().filter(|&c| matches!(c, Cell::Beacon)).count();

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);

    let output = 0;

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day19.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day19 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_19_1_1, part_one, 19, 1, 1);
    test_example!(example_19_2_1, part_two, 19, 2, 1);
}
