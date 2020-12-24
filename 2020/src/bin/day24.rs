use std::collections::HashMap;

type Point = euclid::Point3D<i64, euclid::UnknownUnit>;

#[derive(Debug)]
enum Tile {
    Black,
    White,
}

impl Tile {
    fn flip(&self) -> Self {
        match self {
            Tile::Black => Tile::White,
            Tile::White => Tile::Black,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Dir {
    NE,
    E,
    SE,
    SW,
    W,
    NW,
}

impl std::ops::Add<Point> for Dir {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        match self {
            Dir::E => Point::new(rhs.x + 1, rhs.y - 1, rhs.z),
            Dir::W => Point::new(rhs.x - 1, rhs.y + 1, rhs.z),
            Dir::NW => Point::new(rhs.x, rhs.y + 1, rhs.z - 1),
            Dir::NE => Point::new(rhs.x + 1, rhs.y, rhs.z - 1),
            Dir::SE => Point::new(rhs.x, rhs.y - 1, rhs.z + 1),
            Dir::SW => Point::new(rhs.x - 1, rhs.y, rhs.z + 1),
        }
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Vec<Dir>> + 'a {
    input.lines().map(|l| {
        let mut i = 0;
        let mut dirs = Vec::new();
        while i < l.len() {
            let c = l.chars().nth(i).unwrap();
            let n = l.chars().nth(i + 1);
            match (c, n) {
                ('s', Some('w')) => {
                    i += 1;
                    dirs.push(Dir::SW)
                }
                ('s', Some('e')) => {
                    i += 1;
                    dirs.push(Dir::SE)
                }
                ('n', Some('w')) => {
                    i += 1;
                    dirs.push(Dir::NW)
                }
                ('n', Some('e')) => {
                    i += 1;
                    dirs.push(Dir::NE)
                }
                ('e', _) => dirs.push(Dir::E),
                ('w', _) => dirs.push(Dir::W),
                _ => unreachable!(),
            }
            i += 1;
        }

        dirs
    })
}

fn part_one(input: &str) -> String {
    let mut visited: HashMap<Point, Tile> = HashMap::new();
    for instr in parse_input(input) {
        let p = instr.iter().fold(Point::zero(), |p, &d| d + p);
        let t = visited.entry(p).or_insert(Tile::White);
        *t = t.flip();
    }

    println!("{:?}", visited);
    visited
        .values()
        .filter(|t| match t {
            Tile::Black => true,
            _ => false,
        })
        .count()
        .to_string()
}

fn part_two(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = include_str!("../../input/day24.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 24, 1, 1);
    test_example!(example_two_1, part_two, 24, 2, 1);

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input("esenee").next().unwrap(),
            vec![Dir::E, Dir::SE, Dir::NE, Dir::E]
        );
    }

    #[test]
    fn test_dir() {
        let dirs = parse_input("nwwswee").next().unwrap();
        let start = Point::new(3, 7, 11);
        let end = dirs.iter().fold(start, |p, &d| d + p);
        assert_eq!(start, end);
    }
}
