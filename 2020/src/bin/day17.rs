use euclid;
use std::collections::HashMap;

type Grid = HashMap<Point3D, Cube>;
type Point3D = euclid::Point3D<i64, euclid::UnknownUnit>;

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

fn neighbours(g: &Grid, p: &Point3D) -> Vec<(Point3D, Cube)> {
    (-1..=1)
        .flat_map(move |z| {
            (-1..=1)
                .flat_map(move |y| (-1..=1).map(move |x| Point3D::new(p.x + x, p.y + y, p.z + z)))
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

fn run_cycle(grid: &mut Grid) {
    let prev = grid.clone();

    let (lower_bound, upper_bound) = get_bounds(&prev);
    for z in (lower_bound.z - 1)..=(upper_bound.z + 1) {
        for y in (lower_bound.y - 1)..=(upper_bound.y + 1) {
            for x in (lower_bound.x - 1)..=(upper_bound.x + 1) {
                let p = Point3D::new(x, y, z);
                let c = prev.get(&p).unwrap_or(&Cube::Inactive);
                let ns = neighbours(&prev, &p);
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
        run_cycle(&mut grid);
    }

    grid.iter()
        .filter(|(_, c)| c.is_active())
        .count()
        .to_string()
}

fn part_two(_input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = include_str!("../../input/day17.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 17, 1, 1);
    //test_example!(example_two_1, part_two, 17, 2, 1);
}
