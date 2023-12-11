use anyhow::{anyhow, Result};
use aocutil::{Point, MASK_ALL};
use std::convert::TryFrom;

#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum Tile {
    Floor,
    Occupied,
    #[default]
    Empty,
}

type Grid = aocutil::Grid<Tile>;

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Occupied),
            '.' => Ok(Tile::Floor),
            'L' => Ok(Tile::Empty),
            _ => Err(anyhow!("unknown tile '{}'", value)),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => '.',
                Tile::Occupied => '#',
                Tile::Empty => 'L',
            }
        )
    }
}

fn ray(g: &Grid, p: &Point, (x, y): (i64, i64)) -> Option<(Point, Tile)> {
    let mut next = Point::new(p.x + x, p.y + y);

    while let Some(t) = g.get(&next) {
        match &t {
            Tile::Floor => {
                next.x += x;
                next.y += y;
                continue;
            }
            _ => return Some((next, *t)),
        }
    }

    None
}

fn visible(g: &Grid, p: &Point) -> Vec<(Point, Tile)> {
    let rays = vec![
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
    ];

    let mut seats = Vec::with_capacity(8);

    for r in rays {
        if let Some(v) = ray(g, p, r) {
            seats.push(v)
        }
    }

    seats
}

fn parse_input(s: &str) -> Result<Grid> {
    s.parse()
}

fn part_one(input: &str) -> String {
    let mut prev = parse_input(input).unwrap();

    loop {
        let mut grid = prev.clone();

        for (&p, t) in prev.iter() {
            let occupied = prev
                .surrounding(&p, MASK_ALL)
                .filter(|(_, t)| matches!(t, Tile::Occupied))
                .count();

            grid.insert(
                p,
                match t {
                    Tile::Empty => {
                        if occupied == 0 {
                            Tile::Occupied
                        } else {
                            *t
                        }
                    }
                    Tile::Occupied => {
                        if occupied >= 4 {
                            Tile::Empty
                        } else {
                            *t
                        }
                    }
                    Tile::Floor => *t,
                },
            );
        }

        if grid == prev {
            return grid
                .iter()
                .filter(|(_, &s)| matches!(s, Tile::Occupied))
                .count()
                .to_string();
        }
        prev = grid;
    }
}

fn part_two(input: &str) -> String {
    let mut prev = parse_input(input).unwrap();

    loop {
        let mut grid = prev.clone();

        for (&p, t) in prev.iter() {
            let occupied = visible(&prev, &p)
                .iter()
                .filter(|(_, t)| matches!(t, Tile::Occupied))
                .count();

            grid.insert(
                p,
                match t {
                    Tile::Empty => {
                        if occupied == 0 {
                            Tile::Occupied
                        } else {
                            *t
                        }
                    }
                    Tile::Occupied => {
                        if occupied >= 5 {
                            Tile::Empty
                        } else {
                            *t
                        }
                    }
                    Tile::Floor => *t,
                },
            );
        }

        if grid == prev {
            return grid
                .iter()
                .filter(|(_, &s)| matches!(s, Tile::Occupied))
                .count()
                .to_string();
        }
        prev = grid;
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day11.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 11, 1, 1);
    test_example!(example_two_1, part_two, 11, 2, 1);
}
