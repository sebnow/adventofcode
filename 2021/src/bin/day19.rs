use itertools::Itertools;
use nalgebra::{Point3, Rotation3, Vector3};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

const RANGE: usize = 1000;
const BEACON_OVERLAP: usize = 12;

type Num = i64;
type DistanceMap = HashMap<Num, (Point, Point)>;
type Grid = HashMap<Point, Cell>;
type Point = nalgebra::Point3<Num>;

enum Cell {
    Beacon,
    Scanner(Num),
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Point>,
    distances: DistanceMap,
}

impl Scanner {
    pub fn new(beacons: Vec<Point>) -> Self {
        Scanner {
            distances: Self::get_distances(&beacons),
            beacons,
        }
    }

    pub fn normalize(&self, basis: &Self, required_overlap: usize) -> Option<Self> {
        //let mut candidates: HashMap<_, usize> = HashMap::new();
        let overlap: HashSet<_> = {
            let a: HashSet<_> = self.distances.keys().collect();
            let b: HashSet<_> = basis.distances.keys().collect();
            a.intersection(&b).copied().collect()
        };

        if overlap.len() < required_overlap {
            return None;
        }

        let transforms = [
            Rotation3::new(Vector3::x() * 90.0),
            Rotation3::new(Vector3::x() * 180.0),
            Rotation3::new(Vector3::x() * 270.0),
            Rotation3::new(Vector3::y() * 90.0),
            Rotation3::new(Vector3::y() * 180.0),
            Rotation3::new(Vector3::y() * 270.0),
            Rotation3::new(Vector3::z() * 90.0),
            Rotation3::new(Vector3::z() * 180.0),
            Rotation3::new(Vector3::z() * 270.0),
        ];

        for overlap_d in overlap {
            let (self_b1, self_b2) = self.distances[overlap_d];
            let (basis_b1, basis_b2) = basis.distances[overlap_d];

            for transform in &transforms {
                let rot_b1 = pf_to_pi(&(*transform * pi_to_pf(&self_b1)));
                let rot_b2 = pf_to_pi(&(*transform * pi_to_pf(&self_b2)));

                if rot_b1 - basis_b1 == rot_b2 - basis_b2 {
                    println!("found match");
                }
            }
        }

        None
    }

    fn get_distances(beacons: &[Point]) -> DistanceMap {
        beacons
            .iter()
            .flat_map(move |a| {
                beacons.iter().filter_map(move |b| {
                    if a == b {
                        None
                    } else {
                        let d = distance(a, b);
                        Some((d * d, (*a, *b)))
                    }
                })
            })
            .collect()
    }
}

impl FromIterator<Point> for Scanner {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Scanner::new(iter.into_iter().collect())
    }
}

fn pi_to_pf(p: &Point) -> Point3<f64> {
    Point3::new(p.x as f64, p.y as f64, p.z as f64)
}

fn pf_to_pi(p: &Point3<f64>) -> Point {
    Point3::new(p.x as i64, p.y as i64, p.z as i64)
}

fn distance(a: &Point, b: &Point) -> Num {
    (((b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2)) as f64).sqrt() as Num
}

fn normalize<I: IntoIterator<Item = Scanner>>(
    scanners: I,
    required_overlap: usize,
) -> Vec<Scanner> {
    let mut iter = scanners.into_iter();
    let mut normalized = vec![iter.next().unwrap()];
    let mut queue: VecDeque<_> = iter.collect();

    while let Some(sq) = queue.pop_front() {
        for n_idx in 0..normalized.len() {
            if let Some(n) = sq.normalize(&normalized[n_idx], required_overlap) {
                normalized.push(n);
                continue;
            }
        }

        queue.push_back(sq);
    }

    normalized
}

fn parse_input(s: &str) -> impl Iterator<Item = Scanner> + '_ {
    s.split("\n\n").map(move |block| {
        block
            .lines()
            .skip(1)
            .map(move |l| {
                let mut coords = l.split(',').map(|x| x.parse().unwrap());
                Point::new(
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                )
            })
            .collect()
    })
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let normalized = normalize(input, BEACON_OVERLAP);
    //let grid = reconstruct(input, BEACON_OVERLAP);

    //let output = grid.values().filter(|&c| matches!(c, Cell::Beacon)).count();
    let output = 0;

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
