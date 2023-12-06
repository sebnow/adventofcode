use euclid;
use std::collections::HashMap;

type Grid = HashMap<Point3D, Cube>;
type Grid4D = HashMap<Point4D, Cube>;
type Point3D = euclid::Point3D<i64, euclid::UnknownUnit>;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point4D {
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Point4D { x, y, z, w }
    }

    pub fn min(&self, other: Self) -> Self {
        Point4D::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    pub fn max(&self, other: Self) -> Self {
        Point4D::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Cube {
    Active,
    Inactive,
}

impl Cube {
    fn is_active(&self) -> bool {
        match self {
            Cube::Active => true,
            Cube::Inactive => false,
        }
    }
}

fn neighbours3(g: &Grid, p: &Point3D) -> Vec<(Point3D, Cube)> {
    (-1..=1)
        .flat_map(move |z| {
            (-1..=1)
                .flat_map(move |y| (-1..=1).map(move |x| Point3D::new(p.x + x, p.y + y, p.z + z)))
        })
        .filter(|n| n != p)
        .map(move |n| (n, g.get(&n).map(move |c| *c).unwrap_or(Cube::Inactive)))
        .collect()
}

fn neighbours4(g: &Grid4D, p: &Point4D) -> Vec<(Point4D, Cube)> {
    (-1..=1)
        .flat_map(move |w| {
            (-1..=1).flat_map(move |z| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).map(move |x| Point4D::new(p.x + x, p.y + y, p.z + z, p.w + w))
                })
            })
        })
        .filter(|n| n != p)
        .map(move |n| (n, g.get(&n).map(move |c| *c).unwrap_or(Cube::Inactive)))
        .collect()
}

fn get_bounds(g: &Grid) -> (Point3D, Point3D) {
    g.iter().filter(|(_, c)| c.is_active()).fold(
        (
            Point3D::new(std::i64::MAX, std::i64::MAX, std::i64::MAX),
            Point3D::new(std::i64::MIN, std::i64::MIN, std::i64::MIN),
        ),
        |(l, u), (p, _)| (l.min(*p), u.max(*p)),
    )
}

fn get_bounds4(g: &Grid4D) -> (Point4D, Point4D) {
    g.iter().filter(|(_, c)| c.is_active()).fold(
        (
            Point4D::new(std::i64::MAX, std::i64::MAX, std::i64::MAX, std::i64::MAX),
            Point4D::new(std::i64::MIN, std::i64::MIN, std::i64::MIN, std::i64::MIN),
        ),
        |(l, u), (p, _)| (l.min(*p), u.max(*p)),
    )
}

fn run_cycle3(grid: &mut Grid) {
    let prev = grid.clone();

    let (lower_bound, upper_bound) = get_bounds(&prev);
    for z in (lower_bound.z - 1)..=(upper_bound.z + 1) {
        for y in (lower_bound.y - 1)..=(upper_bound.y + 1) {
            for x in (lower_bound.x - 1)..=(upper_bound.x + 1) {
                let p = Point3D::new(x, y, z);
                let c = prev.get(&p).unwrap_or(&Cube::Inactive);
                let ns = neighbours3(&prev, &p);
                let nc = ns.iter().filter(|(_, nc)| nc.is_active()).count();

                let new = match c {
                    Cube::Active => {
                        if nc == 2 || nc == 3 {
                            Cube::Active
                        } else {
                            Cube::Inactive
                        }
                    }
                    Cube::Inactive => {
                        if nc == 3 {
                            Cube::Active
                        } else {
                            Cube::Inactive
                        }
                    }
                };

                grid.insert(p, new);
            }
        }
    }
}

fn run_cycle4(grid: &mut Grid4D) {
    let prev = grid.clone();

    let (lower_bound, upper_bound) = get_bounds4(&prev);
    for w in (lower_bound.w - 1)..=(upper_bound.w + 1) {
        for z in (lower_bound.z - 1)..=(upper_bound.z + 1) {
            for y in (lower_bound.y - 1)..=(upper_bound.y + 1) {
                for x in (lower_bound.x - 1)..=(upper_bound.x + 1) {
                    let p = Point4D::new(x, y, z, w);
                    let c = prev.get(&p).unwrap_or(&Cube::Inactive);
                    let ns = neighbours4(&prev, &p);
                    let nc = ns.iter().filter(|(_, nc)| nc.is_active()).count();

                    let new = match c {
                        Cube::Active => {
                            if nc == 2 || nc == 3 {
                                Cube::Active
                            } else {
                                Cube::Inactive
                            }
                        }
                        Cube::Inactive => {
                            if nc == 3 {
                                Cube::Active
                            } else {
                                Cube::Inactive
                            }
                        }
                    };

                    grid.insert(p, new);
                }
            }
        }
    }
}

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item = (Point3D, Cube)> + 'a {
    s.lines().enumerate().flat_map(move |(y, l)| {
        l.chars().enumerate().map(move |(x, c)| {
            (
                Point3D::new(x as i64, 0 - y as i64, 0),
                match c {
                    '#' => Cube::Active,
                    '.' => Cube::Inactive,
                    _ => panic!("invalid cube"),
                },
            )
        })
    })
}

fn part_one(input: &str) -> String {
    let mut grid: Grid = parse_input(input).collect();

    for _ in 0..6 {
        run_cycle3(&mut grid);
    }

    grid.iter()
        .filter(|(_, c)| c.is_active())
        .count()
        .to_string()
}

fn part_two(input: &str) -> String {
    let mut grid: Grid4D = parse_input(input)
        .map(|(p, c)| (Point4D::new(p.x, p.y, p.z, 0), c))
        .collect();

    for _ in 0..6 {
        run_cycle4(&mut grid);
    }

    grid.iter()
        .filter(|(_, c)| c.is_active())
        .count()
        .to_string()
}

fn main() {
    let input = include_str!("../../../../input/2020/day17.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 17, 1, 1);
}
