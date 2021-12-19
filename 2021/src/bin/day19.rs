use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

const RANGE: usize = 1000;
const BEACON_OVERLAP: usize = 12;

type Num = i64;
type Distance = Num;
type Point = nalgebra::Point3<Num>;
type Matrix = nalgebra::Matrix3<Num>;
type DistanceMap = HashMap<Distance, (Point, Point)>;
type Grid = HashMap<Point, Cell>;

enum Cell {
    Beacon,
    Scanner(Num),
}

#[derive(Clone, Debug)]
struct Scanner {
    id: Num,
    beacons: Vec<Point>,
}

fn debug(msg: &str) {
    println!("{}", msg);
}

fn distance(a: &Point, b: &Point) -> Distance {
    (((b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2)) as f64).sqrt() as Distance
}

fn find_distances(ps: &[Point]) -> impl Iterator<Item = (Distance, (Point, Point))> + '_{
    ps.iter().flat_map(move |a| {
        ps.iter()
            .filter(move |b| &a != b)
            .map(move |b| (distance(a, b), (*a, *b)))
    })
}

fn reconstruct<I: Iterator<Item = Scanner>>(scanners: I, overlap_requirement: usize) -> Grid {
    #[inline]
    fn add_scanner(grid: &mut Grid, distances: &mut DistanceMap, p: &Point, s: &Scanner) {
        distances.extend(find_distances(&s.beacons));
        grid.insert(*p, Cell::Scanner(s.id));
        grid.extend(s.beacons.iter().map(|p| (*p, Cell::Beacon)));
    }

    let mut queue: VecDeque<_> = scanners.collect();
    let mut distances = DistanceMap::new();
    let mut grid = Grid::new();
    let s = queue.pop_front().unwrap();
    add_scanner(&mut grid, &mut distances, &[0, 0, 0].into(), &s);

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
            add_scanner(&mut grid, &mut distances, &[0, 0, 0].into(), &s);
            continue;
        }

        queue.push_back(s);
    }

    grid
}

fn parse_input(s: &str) -> impl Iterator<Item = Scanner> + '_ {
    s.split("\n\n").enumerate().map(|(id, block)| Scanner {
        id: id as Num,
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
