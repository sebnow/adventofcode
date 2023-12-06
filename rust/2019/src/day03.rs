use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;

        write!(
            f,
            "{}",
            match self {
                Up => "U",
                Right => "R",
                Down => "D",
                Left => "L",
            }
        )
    }
}

#[derive(Debug, Hash)]
pub struct Vector {
    dir: Direction,
    distance: i64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }

    pub fn path(&self, v: &Vector) -> Vec<Point> {
        (0..v.distance)
            .map(|d| match v.dir {
                Direction::Up => Point {
                    x: self.x,
                    y: self.y + d,
                },
                Direction::Down => Point {
                    x: self.x,
                    y: self.y - d,
                },
                Direction::Left => Point {
                    x: self.x - d,
                    y: self.y,
                },
                Direction::Right => Point {
                    x: self.x + d,
                    y: self.y,
                },
            })
            .collect()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<&Vector> for Point {
    fn from(v: &Vector) -> Self {
        match v.dir {
            Direction::Up => Point {
                x: 0,
                y: v.distance,
            },
            Direction::Right => Point {
                x: v.distance,
                y: 0,
            },
            Direction::Down => Point {
                x: 0,
                y: -v.distance,
            },
            Direction::Left => Point {
                x: -v.distance,
                y: 0,
            },
        }
    }
}

impl std::str::FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, l) = s.split_at(1);

        let dir = match d {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(anyhow!("invalid direction: {}", d)),
        }?;

        let distance: i64 = l.parse()?;

        Ok(Vector { dir, distance })
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.dir, self.distance)
    }
}

fn add_path(paths: &mut HashMap<Point, u8>, vs: &[Vector], tag: u8) {
    let mut point = Point { x: 0, y: 0 };

    for v in vs {
        let path = point.path(v);
        for p in path {
            let entry = paths.entry(p).or_insert(tag);
            *entry |= tag;
        }

        point = point + v.into();
    }
}

fn is_point_on_path(p: &Point, a: &Point, b: &Point) -> bool {
    if a.x == p.x {
        b.x == p.x
    } else if a.y == p.y {
        b.y == p.y
    } else {
        (a.x - p.x) * (a.y - p.y) == (p.x - b.x) * (p.y - b.y)
    }

    //    if p.x == a.x && a.x == b.x && a.y <= p.y && p.y <= b.y {
    //        true
    //    } else if p.y == a.y && a.y == b.y && a.x <= p.x && p.x <= b.x {
    //        true
    //    } else {
    //        false
    //    }
}

fn steps_till(paths: &[Vector], p: &Point) -> Option<i64> {
    let mut steps = 0;

    let mut a = Point { x: 0, y: 0 };
    for path in paths {
        let d: Point = path.into();
        let b = Point {
            x: a.x + d.x,
            y: a.y + d.y,
        };

        if is_point_on_path(&p, &a, &b) {
            return Some(steps + a.manhattan_distance(&p) as i64);
        } else {
            steps += path.distance;
            a = b;
        }
    }

    None
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<Vector>> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
        .collect()
}

#[aoc(day3, part1)]
fn answer_1(input: &[Vec<Vector>]) -> Option<u64> {
    let mut paths = HashMap::new();

    add_path(&mut paths, &input[0], 1 << 1);
    add_path(&mut paths, &input[1], 1 << 2);

    paths
        .iter()
        .filter(|(p, &visits)| visits > 4 && !(p.x == 0 && p.y == 0))
        .map(|(p, _)| {
            let zero = Point { x: 0, y: 0 };
            zero.manhattan_distance(&p)
        })
        .min()
}

#[aoc(day3, part2)]
fn answer_2(input: &[Vec<Vector>]) -> Option<i64> {
    let mut paths = HashMap::new();

    add_path(&mut paths, &input[0], 1 << 1);
    add_path(&mut paths, &input[1], 1 << 2);

    paths
        .iter()
        .filter(|(p, &visits)| visits > 4 && !(p.x == 0 && p.y == 0))
        .map(|(p, _)| steps_till(&input[0], p).unwrap() + steps_till(&input[1], p).unwrap())
        .min()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(
            Some(6),
            answer_1(&input_generator(
                r#"R8,U5,L5,D3
U7,R6,D4,L4"#
            ))
        );
        assert_eq!(
            Some(159),
            answer_1(&input_generator(
                r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#
            ))
        );
        assert_eq!(
            Some(135),
            answer_1(&input_generator(
                r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#
            ))
        );
    }

    #[test]
    fn examples_2() {
        assert_eq!(
            Some(30),
            answer_2(&input_generator(
                r#"R8,U5,L5,D3
U7,R6,D4,L4"#
            ))
        );
        assert_eq!(
            Some(610),
            answer_2(&input_generator(
                r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#
            ))
        );
        assert_eq!(
            Some(410),
            answer_2(&input_generator(
                r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#
            ))
        );
    }
}
