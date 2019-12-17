use crate::intcode;
use anyhow::{anyhow, Result};

type Tile = i64;
const SCAFFOLD: Tile = b'#' as i64;
const OPEN_SPACE: Tile = b'.' as i64;
const NL: Tile = b'\n' as i64;

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

fn get_neighbours(map: &[Vec<Tile>], x: usize, y: usize) -> Vec<Tile> {
    let x = x as i64;
    let y = y as i64;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .iter()
        .filter_map(|&(nx, ny)| {
            if nx < 0 || ny < 0 || ny >= map.len() as i64 || nx >= map[ny as usize].len() as i64 {
                None
            } else {
                Some(map[ny as usize][nx as usize])
            }
        })
        .collect()
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

            if get_neighbours(&map, x, y).iter().all(|&t| t == SCAFFOLD) {
                parameters += x * y;
            }
        }
    }
    Ok(parameters)
}

#[aoc(day17, part2)]
fn answer_2(input: &[i64]) -> Result<usize> {
    let is_continuous = std::env::var("DEBUG").map(|x| x != "").unwrap_or(false);

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
