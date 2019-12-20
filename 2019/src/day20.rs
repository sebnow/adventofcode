use anyhow::{anyhow, Result};
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;

type Point = aocutil::Point<usize>;
type Label = String;

#[derive(Debug)]
enum Tile {
    Path,
    Portal(Label),
}

#[derive(Debug)]
struct Maze {
    graph: Graph<Tile, usize>,
    start: NodeIndex,
    goal: NodeIndex,
}

impl Maze {
    pub fn from_grid(grid: &[Vec<char>]) -> Result<Maze> {
        let mut node_ids = HashMap::new();
        let mut graph = Graph::new();
        let mut start = None;
        let mut goal = None;

        for (y, r) in grid.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                let p = Point::new(x, y);

                let tile = match c {
                    '.' => Tile::Path,
                    'A'..='Z' | '#' | ' ' => continue,
                    _ => return Err(anyhow!("unexpected tile {}", c)),
                };

                let node_id = graph.add_node(tile);
                node_ids.insert(p, node_id);

                if let Some((label_p, label)) = find_neighbour_label(grid, &p) {
                    let l = label.to_owned();
                    let label_id = node_ids
                        .entry(label_p)
                        .or_insert_with(|| graph.add_node(Tile::Portal(label)));

                    if l == "AA" {
                        start = Some(label_id.clone());
                    }
                    if l == "ZZ" {
                        goal = Some(label_id.clone());
                    }

                    graph.update_edge(node_id, *label_id, 0);
                }

                if let Some(path_p) = find_neighbour_path(grid, &p) {
                    let path_id = node_ids
                        .entry(path_p)
                        .or_insert_with(|| graph.add_node(Tile::Path));
                    graph.update_edge(node_id, *path_id, 1);
                }
            }
        }

        Ok(Maze {
            graph,
            start: start.ok_or_else(|| anyhow!("start label not found"))?,
            goal: goal.ok_or_else(|| anyhow!("end label not found"))?,
        })
    }

    pub fn shortest_path_cost(&self) -> usize {
        let costs = dijkstra(&self.graph, self.start, Some(self.goal), |e| *e.weight());
        let cost = costs.get(&self.goal).expect("path not found");

        *cost
    }
}

fn find_neighbour_path(grid: &[Vec<char>], p: &Point) -> Option<Point> {
    neighbours(grid, p)
        .iter()
        .find(|&p| grid[p.y][p.x] == '.')
        .copied()
}

fn find_neighbour_letter(grid: &[Vec<char>], p: &Point) -> Option<(Point, char)> {
    neighbours(grid, p).iter().find_map(|&p| {
        let c = grid[p.y][p.x];
        match c {
            'A'..='Z' => Some((p, c)),
            _ => None,
        }
    })
}

fn find_neighbour_label(grid: &[Vec<char>], p: &Point) -> Option<(Point, Label)> {
    if let Some((fst_p, fst_c)) = find_neighbour_letter(grid, p) {
        if let Some((snd_p, snd_c)) = find_neighbour_letter(grid, &fst_p) {
            return Some((
                fst_p,
                if fst_p.x < snd_p.x || fst_p.y > snd_p.y {
                    format!("{}{}", fst_c, snd_c)
                } else {
                    format!("{}{}", snd_c, fst_c)
                },
            ));
        }
    }

    None
}

fn neighbours(grid: &[Vec<char>], p: &Point) -> Vec<Point> {
    [
        Point::new(p.x - 1, p.y),
        Point::new(p.x + 1, p.y),
        Point::new(p.x, p.y - 1),
        Point::new(p.x, p.y + 1),
    ]
    .iter()
    .filter_map(|&p| {
        if (p.y as usize) < grid.len() && (p.x as usize) < grid[p.y as usize].len() {
            Some(p)
        } else {
            None
        }
    })
    .collect()
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    let mut row = Vec::new();
    for r in input.lines() {
        for c in r.chars() {
            row.push(c);
        }
        let len = row.len();
        grid.push(row);
        row = Vec::with_capacity(len);
    }

    grid
}

#[aoc(day20, part1)]
fn answer_1(input: &[Vec<char>]) -> Result<usize> {
    let maze = Maze::from_grid(input)?;
    Ok(maze.shortest_path_cost())
}

#[aoc(day20, part2)]
fn answer_2(input: &[Vec<char>]) -> Result<usize> {
    Ok(0)
}
