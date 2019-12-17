use crate::intcode;
use anyhow::{anyhow, Result};
use aocutil::Direction;
use aocutil::Grid;
use std::collections::VecDeque;

const SCAFFOLD: Tile = Tile(b'#' as i64);
const OPEN_SPACE: Tile = Tile(b'.' as i64);
const NL: i64 = b'\n' as i64;

type Point = aocutil::Point<i64>;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Tile(i64);

impl Tile {
    pub fn new(code: i64) -> Self {
        Tile(code)
    }

    pub fn is_walkable(&self) -> bool {
        match self.as_char() {
            '^' | '>' | '<' | 'v' | 'X' | '#' => true,
            _ => false,
        }
    }

    pub fn as_char(&self) -> char {
        (self.0 as u8) as char
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Debug)]
struct Turn {
    relative: char,
    absolute: Direction,
}

impl Turn {
    pub fn new(abs: Direction, rel: char) -> Self {
        Turn {
            absolute: abs,
            relative: rel,
        }
    }

    pub fn try_from_points(dir: Direction, a: Point, b: Point) -> Option<Turn> {
        use Direction::*;

        let (dx, dy) = (b.x - a.x, b.y - a.y);

        match (dir, dx, dy) {
            (Up, 1, 0) => Some(Turn::new(Right, 'R')),
            (Up, -1, 0) => Some(Turn::new(Left, 'L')),
            (Right, 0, 1) => Some(Turn::new(Up, 'L')),
            (Right, 0, -1) => Some(Turn::new(Down, 'R')),
            (Down, 1, 0) => Some(Turn::new(Right, 'L')),
            (Down, -1, 0) => Some(Turn::new(Left, 'R')),
            (Left, 0, 1) => Some(Turn::new(Up, 'R')),
            (Left, 0, -1) => Some(Turn::new(Down, 'L')),
            _ => None,
        }
    }
}

fn build_map(input: &[i64]) -> Result<Vec<Vec<Tile>>> {
    let mut prg = intcode::Interpretor::new(input);
    let mut map = Vec::new();
    let mut current = Vec::new();

    loop {
        match prg.run()? {
            intcode::State::Suspended(o) => match o {
                NL => {
                    let len = current.len();
                    map.push(current);
                    current = Vec::with_capacity(len);
                }
                _ => {
                    current.push(Tile::new(o));
                }
            },
            intcode::State::AwaitingInput => return Err(anyhow!("unexpectedly awaiting input")),
            intcode::State::Terminated(_) => break,
        }
    }

    Ok(map)
}

fn build_grid(input: &[i64]) -> Result<Grid<Tile>> {
    let map = build_map(input)?;
    let height = map.len();
    let mut grid = Grid::new();
    for (y, r) in map.iter().enumerate() {
        for (x, &t) in r.iter().enumerate() {
            grid.add(Point::new(x as i64, (height - y) as i64), t);
        }
    }

    Ok(grid)
}

fn get_neighbours(grid: &Grid<Tile>, p: Point) -> Vec<(Point, Tile)> {
    [
        Point::new(p.x - 1, p.y),
        Point::new(p.x + 1, p.y),
        Point::new(p.x, p.y - 1),
        Point::new(p.x, p.y + 1),
    ]
    .iter()
    .filter_map(|&adjacent| grid.at(adjacent).map(|&t| (adjacent, t)))
    .collect()
}

fn get_neighbour_scaffolds(grid: &Grid<Tile>, p: Point) -> Vec<(Point, Tile)> {
    get_neighbours(grid, p)
        .iter()
        .filter(|(_, t)| *t == SCAFFOLD)
        .copied()
        .collect()
}

#[derive(Clone, Debug)]
struct Robot {
    path: Vec<String>,
    direction: Direction,
    position: Point,
}

impl Robot {
    pub fn new(p: Point, d: Direction) -> Self {
        Robot {
            path: Vec::default(),
            direction: d,
            position: p,
        }
    }
}

fn find_robots(grid: &Grid<Tile>) -> Vec<Robot> {
    for (&p, &t) in grid.points.iter() {
        let robots = match t.as_char() {
            '^' => vec![Robot::new(p, Direction::Up)],
            '>' => vec![Robot::new(p, Direction::Right)],
            '<' => vec![Robot::new(p, Direction::Left)],
            'v' => vec![Robot::new(p, Direction::Down)],
            'X' => vec![
                Robot::new(p, Direction::Up),
                Robot::new(p, Direction::Right),
                Robot::new(p, Direction::Down),
                Robot::new(p, Direction::Left),
            ],
            _ => continue,
        };

        return robots;
    }

    vec![]
}

fn find_paths(grid: &Grid<Tile>) -> Vec<String> {
    let mut paths = Vec::new();
    let mut robots: VecDeque<Robot> = find_robots(grid).into();

    while let Some(robot) = robots.pop_front() {
        let tiles = get_neighbour_scaffolds(grid, robot.position);
        if tiles.is_empty() {
            panic!("orphaned?");
        }

        if tiles.len() == 1 {
            let back = match robot.direction {
                Direction::Up => Direction::Down,
                Direction::Right => Direction::Left,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Left,
            };
            // Can't go back so this path is complete
            if robot.position + back.into() == tiles[0].0 {
                paths.push(robot.path.join(","));
                continue;
            }
        }

        // Queue other paths
        for (next_pos, _) in tiles {
            let mut new_robot = robot.clone();
            let turn = Turn::try_from_points(new_robot.direction, new_robot.position, next_pos);
            // Can only go left or right
            if turn.is_none() {
                continue;
            }
            let turn = turn.unwrap();

            let mut steps = 0;
            let mut pos = new_robot.position + turn.absolute.into();
            while let Some(t) = grid.at(pos) {
                if !t.is_walkable() {
                    break;
                }

                pos = pos + turn.absolute.into();
                steps += 1;
            }

            assert!(steps > 0, "appending path when no steps were taken");
            new_robot.position = pos - turn.absolute.into();
            new_robot.direction = turn.absolute;
            new_robot.path.push(format!("{},{}", turn.relative, steps));
            robots.push_back(new_robot);
        }
    }

    paths
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day17, part1)]
fn answer_1(input: &[i64]) -> Result<usize> {
    let grid = build_grid(input)?;

    let mut parameters = 0;
    for (&p, &tile) in grid.points.iter() {
        if tile == OPEN_SPACE {
            continue;
        }

        if get_neighbour_scaffolds(&grid, p).len() == 4 {
            parameters += p.x * p.y;
        }
    }
    Ok(parameters as usize)
}

#[aoc(day17, part2)]
fn answer_2(input: &[i64]) -> Result<usize> {
    let is_continuous = std::env::var("DEBUG").map(|x| x != "").unwrap_or(false);
    let grid = build_grid(input)?;

    let paths = find_paths(&grid);
    for path in paths {
        println!("{}", path);
    }

    let mut input = input.to_owned();
    input[0] = 2;

    let mut prg = intcode::Interpretor::new(&input);
    // R,8,L,4,R,4,R,10
    // R,8
    // R,8,L,4,R,4,R,10
    // R,8
    // L,12,L,12
    // R,8
    // R,8
    // R,8
    prg.input_str("A,B,A,B,C,B,B,B\n");
    prg.input_str("R,8,L,4,R,4,R,10\n");
    prg.input_str("R,8\n");
    prg.input_str("L,12,L,12\n");
    prg.input_str(if is_continuous { "y\n" } else { "n\n" });

    loop {
        match prg.run()? {
            intcode::State::Suspended(o) => print!("{}", (o as u8) as char),
            intcode::State::AwaitingInput => return Err(anyhow!("unexpectedly awaiting input")),
            intcode::State::Terminated(dust) => {
                println!("{:?}", dust);
                break;
            }
        }
    }

    println!();
    Ok(0)
}
