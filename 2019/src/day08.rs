use anyhow::{anyhow, Result};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).expect("want digit") as u8))
        .flatten()
        .collect()
}

pub fn count_value(xs: &[u8], x: u8) -> usize {
    xs.iter().filter(|&y| *y == x).count()
}

#[aoc(day8, part1)]
fn answer_1(input: &[u8]) -> Result<usize> {
    let width = 25;
    let height = 6;
    input
        .chunks(width * height)
        .min_by(|&a, &b| count_value(a, 0).cmp(&count_value(b, 0)))
        .map(|l| count_value(l, 1) * count_value(l, 2))
        .ok_or_else(|| anyhow!("unable to find layer"))
}

#[aoc(day8, part2)]
fn answer_2(input: &[u8]) -> Result<i64> {
    Ok(0)
}
