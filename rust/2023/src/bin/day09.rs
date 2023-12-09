use std::{collections::HashMap, unreachable};

use anyhow::Result;

fn parse_input(s: &str) -> Vec<Vec<i64>> {
    s.lines()
        .map(|l| l.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn next_value(s: &[i64]) -> i64 {
    let mut next = Vec::with_capacity(s.len());
    let mut steps: Vec<Vec<_>> = Vec::new();

    next.extend(s.iter());
    steps.push(next.clone());

    while !next.iter().all(|&x| x == 0) {
        for i in 0..next.len() - 1 {
            next[i] = next[i + 1] - next[i];
        }
        next.remove(next.len() - 1);
        steps.push(next.clone());
    }

    for i in (0..steps.len() - 1).rev() {
        let last = steps[i][steps[i].len() - 1];
        let diff = steps[i + 1][steps[i + 1].len() - 1];
        steps[i].push(last + diff);
    }

    *steps[0].last().unwrap()
}

fn prev_value(s: &[i64]) -> i64 {
    let mut nums = Vec::with_capacity(s.len());
    let mut steps: Vec<Vec<_>> = Vec::new();

    nums.extend(s.iter());
    steps.push(nums.clone());

    while !nums.iter().all(|&x| x == 0) {
        for i in 1..nums.len() {
            nums[i - 1] -= nums[i];
        }
        nums.remove(nums.len() - 1);
        steps.push(nums.clone());
    }

    for i in (0..steps.len() - 1).rev() {
        let first = steps[i][0];
        let diff = steps[i + 1][0];
        steps[i].insert(0, first + diff);
    }

    steps[0][0]
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);

    input.iter().map(|v| next_value(v)).sum::<i64>().to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);

    input.iter().map(|v| prev_value(v)).sum::<i64>().to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day09.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day09 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 9, 1, 1);
    test_example!(example_2_1, part_two, 9, 2, 1);
}
