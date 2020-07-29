use crate::grid::Grid;
use anyhow::{anyhow, Result};

#[derive(Clone, Copy)]
pub enum Tile {
    Wall,
    Space,
    Key(char),
    Door(char),
    Entrance,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Space
    }
}

impl std::str::FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Tile::*;
        let c = s
            .chars()
            .nth(0)
            .ok_or_else(|| anyhow!("missing character"))?;

        match c {
            '#' => Ok(Wall),
            '.' => Ok(Space),
            'a'..='z' => Ok(Key(c)),
            'A'..='Z' => Ok(Door(c)),
            '@' => Ok(Entrance),
            _ => Err(anyhow!("invalid tile '{}'", c)),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tile::*;

        write!(
            f,
            "{}",
            match *self {
                Wall => '#',
                Space => '.',
                Key(c) => c,
                Door(c) => c,
                Entrance => '@',
            }
        )
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|x| x.to_string().parse().unwrap()).collect())
        .collect();

    Grid::from_vec2d(tiles)
}

#[aoc(day18, part1)]
fn answer_1(input: &Grid<Tile>) -> Result<usize> {
    println!("{}", input);
    // DFS
    // Path finding
    Ok(0)
}

#[aoc(day18, part2)]
fn answer_2(input: &Grid<Tile>) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        assert_eq!(
            132,
            answer_1(&input_generator(&include_str!("../examples/day18-1-1.txt"))).unwrap()
        );
    }

    #[test]
    fn test_1_2() {
        assert_eq!(
            136,
            answer_1(&input_generator(&include_str!("../examples/day18-1-2.txt"))).unwrap()
        );
    }

    #[test]
    fn test_1_3() {
        assert_eq!(
            81,
            answer_1(&input_generator(&include_str!("../examples/day18-1-3.txt"))).unwrap()
        );
    }
}
