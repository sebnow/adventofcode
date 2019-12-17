use crate::intcode;
use anyhow::Result;

type Tile = i64;
const SCAFFOLD: Tile = 35;
const OPEN_SPACE: Tile = 46;
const NL: Tile = 10;

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
            intcode::State::AwaitingInput => panic!("wut"),
            intcode::State::Terminated(_) => break,
        }
    }

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
    Ok(0)
}
