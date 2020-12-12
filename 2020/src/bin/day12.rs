use anyhow::{anyhow, Result};
use aocutil::Point;

#[derive(Debug)]
enum Instr {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

impl std::str::FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: i64 = s[1..].parse()?;
        match s.chars().nth(0) {
            Some('N') => Ok(Instr::N(v)),
            Some('S') => Ok(Instr::S(v)),
            Some('E') => Ok(Instr::E(v)),
            Some('W') => Ok(Instr::W(v)),
            Some('L') => Ok(Instr::L(v)),
            Some('R') => Ok(Instr::R(v)),
            Some('F') => Ok(Instr::F(v)),
            _ => Err(anyhow!("invalid instruction {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn left(&self, v: i64) -> Self {
        use Dir::*;
        assert!(v % 90 == 0);

        (0..(v / 90)).fold(*self, |d, _| match d {
            North => West,
            West => South,
            South => East,
            East => North,
        })
    }

    fn right(&self, v: i64) -> Self {
        use Dir::*;
        assert!(v % 90 == 0);

        (0..(v / 90)).fold(*self, |d, _| match d {
            North => East,
            East => South,
            South => West,
            West => North,
        })
    }
}

struct Ship {
    loc: Point,
    dir: Dir,
    wp: Point,
}

fn parse_input(s: &str) -> Result<Vec<Instr>> {
    s.lines().map(|l| l.parse()).collect()
}

fn part_one(input: &str) -> String {
    let ship = parse_input(input).unwrap().iter().fold(
        Ship {
            loc: Point::new(0, 0),
            dir: Dir::East,
            wp: Point::new(0, 0),
        },
        |Ship { loc, dir, wp }, instr| {
            use Instr::*;
            match *instr {
                N(v) => Ship {
                    loc: Point::new(loc.x, loc.y + v),
                    dir,
                    wp,
                },
                S(v) => Ship {
                    loc: Point::new(loc.x, loc.y - v),
                    dir,
                    wp,
                },
                E(v) => Ship {
                    loc: Point::new(loc.x + v, loc.y),
                    dir,
                    wp,
                },
                W(v) => Ship {
                    loc: Point::new(loc.x - v, loc.y),
                    dir,
                    wp,
                },
                L(v) => Ship {
                    loc,
                    dir: dir.left(v),
                    wp,
                },
                R(v) => Ship {
                    loc,
                    dir: dir.right(v),
                    wp,
                },
                F(v) => Ship {
                    loc: match dir {
                        Dir::North => Point::new(loc.x, loc.y + v),
                        Dir::South => Point::new(loc.x, loc.y - v),
                        Dir::East => Point::new(loc.x + v, loc.y),
                        Dir::West => Point::new(loc.x - v, loc.y),
                    },
                    dir,
                    wp,
                },
            }
        },
    );

    (ship.loc.x.abs() + ship.loc.y.abs()).to_string()
}

fn part_two(input: &str) -> String {
    let ship = parse_input(input).unwrap().iter().fold(
        Ship {
            loc: Point::new(0, 0),
            dir: Dir::East,
            wp: Point::new(10, 1),
        },
        |Ship { loc, dir, wp }, instr| {
            use Instr::*;
            match *instr {
                N(v) => Ship {
                    wp: Point::new(wp.x, wp.y + v),
                    dir,
                    loc,
                },
                S(v) => Ship {
                    wp: Point::new(wp.x, wp.y - v),
                    dir,
                    loc,
                },
                E(v) => Ship {
                    wp: Point::new(wp.x + v, wp.y),
                    dir,
                    loc,
                },
                W(v) => Ship {
                    wp: Point::new(wp.x - v, wp.y),
                    dir,
                    loc,
                },
                L(v) => Ship {
                    loc,
                    dir,
                    wp: (0..(v/90)).fold(wp, |w, _| Point::new(w.y * -1, w.x)),
                },
                R(v) => Ship {
                    loc,
                    dir,
                    wp: (0..(v/90)).fold(wp, |w, _| Point::new(w.y, w.x * -1)),
                },
                F(v) => Ship {
                    loc: Point::new(loc.x + wp.x * v, loc.y + wp.y * v),
                    dir,
                    wp,
                },
            }
        },
    );

    (ship.loc.x.abs() + ship.loc.y.abs()).to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day12.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 12, 1, 1);
    test_example!(example_two_1, part_two, 12, 2, 1);
}
