use crate::grid::{Collision, Grid, Point};
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl Collision for Tile {
    fn is_collidable(&self) -> bool {
        use Tile::*;

        match self {
            Wall => true,
            Door(_) => true,
            Key(_) => false,
            Space => false,
            Entrance => false,
        }
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

#[derive(Debug)]
struct Path {
    prev: Point,
    pos: Point,
    grid: Grid<Tile>,
    amount_of_keys_left: usize,
    keys: u32,
    steps: usize,
}

impl Path {
    fn add_key(&mut self, key: char) {
        self.keys = self.keys | 2 << key as u8 - b'a'
    }
    fn has_key_for_door(&self, door: char) -> bool {
        self.has_key(door.to_ascii_lowercase())
    }

    fn has_key(&self, key: char) -> bool {
        self.keys & 2 << key as u8 - b'a' != 0
    }

    fn get_keys(&self) -> Vec<char> {
        ('a'..='z').filter(|&k| self.has_key(k)).collect()
    }
}

fn is_key(t: &Tile) -> bool {
    match t {
        Tile::Key(_) => true,
        _ => false,
    }
}

#[aoc(day18, part1)]
fn answer_1(input: &Grid<Tile>) -> Result<usize> {
    println!("{}", input);
    let mut paths = VecDeque::new();

    let mut grid = input.clone();
    let mut complete_paths = Vec::new();
    {
        let entrances = input.find(&Tile::Entrance);
        assert_eq!(1, entrances.len());
        let entrance = entrances[0];
        let amount_of_keys = input.filter(|&(_, t)| is_key(t)).count();
        assert_ne!(0, amount_of_keys);

        grid.insert(*entrance, Tile::Space);
        paths.push_back(Path {
            prev: *entrance,
            pos: *entrance,
            grid: grid,
            amount_of_keys_left: amount_of_keys,
            keys: 0,
            steps: 0,
        });
    }

    let mut iterations = 0;
    while let Some(mut path) = paths.pop_back() {
        if iterations > 20000 {
            break;
        }

        println!("");
        println!("=========== {} ==========", iterations);
        // TODO: REMOVE
        {
            let mut g = path.grid.clone();
            g.insert(path.pos, Tile::Entrance);

            println!("{}", g);
        }
        println!("prev: {:?}", path.prev);
        println!("pos: {:?}", path.pos);
        println!("steps: {:?}", path.steps);
        println!("keys: {:?}", path.get_keys());
        println!("keys left: {:?}", path.amount_of_keys_left);
        let mut amount_of_keys_left = path.amount_of_keys_left;
        let mut new_grid = path.grid.clone();
        let current_tile = path
            .grid
            .get(&path.pos)
            .ok_or_else(|| anyhow!("strayed off the beaten path"))?;

        if let &Tile::Door(d) = current_tile {
            if path.has_key_for_door(d) {
                new_grid.insert(path.pos, Tile::Space);
            }
        } else if let &Tile::Key(k) = current_tile {
            if !path.has_key(k) {
                println!("picked up {}", k);
                path.add_key(k);
                amount_of_keys_left -= 1;

                // Try going back to check if a previously blocked passage has opened
                paths.push_back(Path {
                    prev: path.pos,
                    pos: path.prev,
                    grid: new_grid.clone(),
                    amount_of_keys_left,
                    keys: path.keys,
                    steps: path.steps + 1,
                });

                if amount_of_keys_left == 0 {
                    complete_paths.push(path);
                    continue;
                }
            }
        }

        let possible = path.grid.filter_surrounding(path.pos, |&p, v| {
            // filter_surround returns positions on the diagonal as well. We want a crosshair
            // pattern only.
            if !(path.pos.x == p.x || path.pos.y == p.y) {
                return false;
            }

            if let &Tile::Door(d) = v {
                return path.has_key_for_door(d);
            }

            !v.is_collidable() && path.prev != p
        });
        println!("possible: {:?}", possible);

        for p in possible {
            paths.push_back(Path {
                prev: path.pos,
                pos: p,
                grid: new_grid.clone(),
                amount_of_keys_left,
                keys: path.keys,
                steps: path.steps + 1,
            })
        }

        iterations += 1;
    }

    complete_paths
        .iter()
        .min_by_key(|p| p.steps)
        .map(|p| p.steps)
        .ok_or_else(|| anyhow!("no paths found"))
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
