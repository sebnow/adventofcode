use crate::intcode;
use anyhow::{anyhow, Result};
use aocutil::Direction;
use std::collections::VecDeque;

type Tile = i64;
const SCAFFOLD: Tile = b'#' as i64;
const OPEN_SPACE: Tile = b'.' as i64;
const NL: Tile = b'\n' as i64;

type Point = aocutil::Point<i64>;

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

        println!("{:?} ({}, {})", dir, dx, dy);
        match (dir, dx, dy) {
            (Right, 0, 1) => Some(Turn::new(Up, 'L')),
            (Right, 0, -1) => Some(Turn::new(Down, 'R')),
            (Left, 0, 1) => Some(Turn::new(Down, 'R')),
            (Left, 0, -1) => Some(Turn::new(Up, 'L')),
            (Up, 1, 0) => Some(Turn::new(Right, 'R')),
            (Up, -1, 0) => Some(Turn::new(Left, 'L')),
            (Down, 1, 0) => Some(Turn::new(Left, 'L')),
            (Down, -1, 0) => Some(Turn::new(Right, 'R')),
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
                    current.push(o);
                }
            },
            intcode::State::AwaitingInput => return Err(anyhow!("unexpectedly awaiting input")),
            intcode::State::Terminated(_) => break,
        }
    }

    Ok(map)
}

fn get_neighbours(map: &[Vec<Tile>], p: Point) -> Vec<(Point, Tile)> {
    [
        (p.x - 1, p.y),
        (p.x + 1, p.y),
        (p.x, p.y - 1),
        (p.x, p.y + 1),
    ]
    .iter()
    .filter_map(|&(nx, ny)| {
        if nx < 0 || ny < 0 || ny >= map.len() as i64 || nx >= map[ny as usize].len() as i64 {
            None
        } else {
            Some((Point::new(nx, ny), map[ny as usize][nx as usize]))
        }
    })
    .collect()
}

fn get_neighbour_scaffolds(map: &[Vec<Tile>], p: Point) -> Vec<(Point, Tile)> {
    get_neighbours(map, p)
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

fn find_robots(map: &[Vec<Tile>]) -> Vec<Robot> {
    for (x, r) in map.iter().enumerate() {
        for (y, t) in r.iter().enumerate() {
            let p = Point::new(x as i64, y as i64);
            let robots = match (*t as u8) as char {
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
    }

    vec![]
}

fn find_paths(map: &[Vec<Tile>]) -> Vec<String> {
    let mut paths = Vec::new();
    let mut robots: VecDeque<Robot> = find_robots(map).into();

    while let Some(robot) = robots.pop_front() {
        let tiles = get_neighbour_scaffolds(&map, robot.position);
        println!("{:?}, {:?}", robot, tiles);
        if tiles.is_empty() {
            panic!("orphaned?");
        }

        if tiles.len() == 1 {
            // Can't go back so this path is complete
            paths.push(robot.path.join(","));
            continue;
        }

        // Queue other paths
        for (next_pos, _) in tiles {
            let mut new_robot = robot.clone();
            println!("{:?}", new_robot);
            let turn = Turn::try_from_points(new_robot.direction, new_robot.position, next_pos);
            if turn.is_none() {
                continue;
            }
            let turn = turn.unwrap();
            let mut steps = 0;

            while map[new_robot.position.y as usize][new_robot.position.x as usize] == SCAFFOLD {
                match turn.absolute {
                    Direction::Up => new_robot.position.y += 1,
                    Direction::Down => new_robot.position.y -= 1,
                    Direction::Left => new_robot.position.x -= 1,
                    Direction::Right => new_robot.position.x += 1,
                }
                steps += 1;
            }

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
    let map = build_map(input)?;

    let mut parameters = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == OPEN_SPACE {
                continue;
            }

            if get_neighbour_scaffolds(&map, Point::new(x as i64, y as i64)).len() == 4 {
                parameters += x * y;
            }
        }
    }
    Ok(parameters)
}

#[aoc(day17, part2)]
fn answer_2(input: &[i64]) -> Result<usize> {
    let is_continuous = std::env::var("DEBUG").map(|x| x != "").unwrap_or(false);
    let map = build_map(input)?;

    let paths = find_paths(&map);
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
            intcode::State::Suspended(o) => {
                print!(
                    "{}",
                    ((if o == OPEN_SPACE { b' ' as i64 } else { o }) as u8) as char
                );
            }
            intcode::State::AwaitingInput => return Err(anyhow!("unexpectedly awaiting input")),
            intcode::State::Terminated(_) => break,
        }
    }

    println!();
    Ok(0)
}
