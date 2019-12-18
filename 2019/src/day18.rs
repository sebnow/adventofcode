use anyhow::{anyhow, Result};
use pathfinding;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Door(char),
    Key(char),
    Path,
    Player,
    Wall,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Door(c) => *c,
                Tile::Key(c) => *c,
                Tile::Path => ' ',
                Tile::Player => '@',
                Tile::Wall => '░',
            }
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Path),
            '@' => Ok(Tile::Player),
            'A'..='Z' => Ok(Tile::Door(c)),
            'a'..='z' => Ok(Tile::Key(c)),
            _ => Err(anyhow!("invalid tile ({})", c)),
        }
    }
}

type Point = aocutil::Point<i64>;
type Grid = aocutil::Grid<Tile>;

fn traversable_tiles<'a>(grid: &'a Grid, p: &Point) -> Vec<(Point, &'a Tile)> {
    grid.adjacent(p)
        .iter()
        .filter(|(_, &t)| t != Tile::Wall)
        .copied()
        .collect()
}

fn unobstructed_distance(grid: &Grid, a: &Point, b: &Point) -> Option<usize> {
    let path = dijkstra(
        a,
        |p| -> Vec<(Point, usize)> {
            traversable_tiles(grid, p)
                .iter()
                .map(|(p, _)| (*p, 1))
                .collect()
        },
        |p| p == b,
    );

    path.map(|r| r.1)
}

fn get_key_distances(grid: &Grid) -> HashMap<Point, Vec<(Point, char, usize)>> {
    let keys: Vec<(Point, char)> = grid
        .iter()
        .filter_map(|(&p, t)| {
            if let Tile::Key(k) = t {
                Some((p, *k))
            } else {
                None
            }
        })
        .collect();
    let mut key_edges = HashMap::with_capacity(keys.len());

    for a in &keys {
        for b in &keys {
            if a == b {
                continue;
            }

            let e = key_edges.entry(a.0).or_insert(Vec::default());
            if let Some(d) = unobstructed_distance(grid, &a.0, &b.0) {
                e.push((b.0, b.1, d));
            }
        }
    }

    key_edges
}

fn find_tile(grid: &Grid, tile: Tile) -> Option<Point> {
    grid.iter().find(|(_, &t)| t == tile).map(|(&p, _)| p)
}

fn build_grid(input: &str) -> Grid {
    let mut g = Grid::default();

    for (y, l) in input.lines().enumerate() {
        let l = l.trim();
        for (x, c) in l.chars().enumerate() {
            g.add(Point::new(x as i64, y as i64), Tile::try_from(c).unwrap());
        }
    }

    g.flip_y();
    g
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> String {
    input.to_owned()
}

#[aoc(day18, part1)]
fn answer_1(input: &str) -> Result<usize> {
    let mut grid = build_grid(input);
    let mut key_distance = get_key_distances(&grid);
    let mut player = find_tile(&grid, Tile::Player).expect("player not found");
    let mut keys = vec![];
    let mut steps = 0;

    println!("{}", grid);
    loop {
        // Find closest key
        let result = dijkstra(
            &player,
            |p| -> Vec<(Point, usize)> {
                grid.adjacent(p)
                    .iter()
                    .filter(|(_, &t)| match t {
                        Tile::Wall | Tile::Door(_) => false,
                        _ => true,
                    })
                    .map(|(p, _)| (*p, 1))
                    .collect()
            },
            |p| key_distance.contains_key(p),
        );
        if result.is_none() {
            break;
        }

        let (path, cost) = result.unwrap();

        // Remove the key and associated door
        let key_pos = path[path.len() - 1];
        let key = grid.remove(&key_pos).unwrap();
        match key {
            Tile::Key(k) => {
                keys.push(k);
                key_distance.remove(&key_pos);
                let door_pos = find_tile(&grid, Tile::Door(k.to_ascii_uppercase()))
                    .ok_or_else(|| anyhow!("door for {} not found", k))?;
                grid.remove(&door_pos);
                Ok(())
            }
            _ => Err(anyhow!("key was not a key")),
        }?;

        // Move the player
        grid.remove(&player);
        player = key_pos;
        grid.add(player, Tile::Player);

        steps += cost;

        println!("Keys: {:?}", keys);
        println!("{}", grid);
        println!();
    }

    Ok(steps)
}

#[aoc(day18, part2)]
fn answer_2(_input: &str) -> Result<usize> {
    Ok(0)
}
