use anyhow::Result;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day25, part1)]
fn answer_1(input: &[i64]) -> Result<usize> {
    Ok(0)
}

#[aoc(day25, part2)]
fn answer_2(input: &[i64]) -> Result<usize> {
    Ok(0)
}
