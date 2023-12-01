use anyhow::{anyhow, Result};
use aocutil::Point;
use std::convert::{TryFrom, TryInto};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Tree,
    Empty,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

type Grid = aocutil::Grid<Tile>;

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Tree),
            '.' => Ok(Tile::Empty),
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
                Tile::Tree => '#',
                Tile::Empty => '.',
            }
        )
    }
}

fn parse_input(s: &str) -> Result<Grid> {
    let mut g = Grid::default();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            g.insert(Point::new(x as i64, 0 - y as i64), c.try_into()?)
        }
    }

    Ok(g)
}

fn hit_some_trees(grid: &Grid, slope: (i64, i64)) -> usize {
    let cols = grid.cols();
    let rows = grid.rows();
    let mut trees_hit = 0;
    let mut pos = Point::new(0, 0);

    while pos.y.abs() < rows as i64 {
        pos.x += slope.0;
        pos.y -= slope.1;

        if pos.x > cols as i64 {
            pos.x -= cols as i64 + 1;
        };

        if let Some(Tile::Tree) = grid.get(&pos) {
            trees_hit += 1;
        }
    }

    trees_hit
}

fn part_one(input: &str) -> String {
    let grid = parse_input(input).unwrap();

    hit_some_trees(&grid, (3, 1)).to_string()
}

fn part_two(input: &str) -> String {
    let grid = parse_input(input).unwrap();

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&slope| hit_some_trees(&grid, slope))
        .product::<usize>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day03.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 3, 1, 1);
    test_example!(example_two_1, part_two, 3, 2, 1);
}
