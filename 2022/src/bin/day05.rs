use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug)]
struct Crate(char);

#[derive(Debug)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(s: &str) -> Result<(Vec<Vec<Crate>>, Vec<MoveInstruction>)> {
    let parts = s
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("malformed input"))?;

    let mut stacks: Vec<Vec<Crate>> = Vec::new();
    for line in parts.0.lines() {
        let mut stack_idx = 0;

        for mut chunk in &line.chars().chunks(4) {
            if stacks.len() < stack_idx + 1 {
                stacks.push(Vec::new());
            }

            match chunk.next() {
                Some('[') => {
                    stacks[stack_idx].insert(0, Crate(chunk.next().unwrap()));
                    stack_idx += 1;
                }
                _ => {
                    stack_idx += 1;
                    continue;
                }
            }
        }
    }

    let instructions: Result<Vec<MoveInstruction>> = parts
        .1
        .lines()
        .map(|l| {
            let mut instr = l.split(' ');
            instr.next();
            let count = instr.next().unwrap().parse()?;
            instr.next();
            let from = instr.next().unwrap().parse()?;
            instr.next();
            let to = instr.next().unwrap().parse()?;

            Ok(MoveInstruction { count, from, to })
        })
        .collect();

    Ok((stacks, instructions?))
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let (mut stacks, instructions) = input;

    for instruction in &instructions {
        for _ in 0..instruction.count {
            let krate = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(krate);
        }
    }

    stacks.iter().map(|s| s[s.len() - 1].0).collect()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let (mut stacks, instructions) = input;

    for instruction in &instructions {
        let stack = stacks.get_mut(instruction.from - 1).unwrap();
        let crates = stack.split_off(stack.len() - instruction.count);
        stacks[instruction.to - 1].extend(crates);
    }

    stacks.iter().map(|s| s[s.len() - 1].0).collect()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day05.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_5_1, part_one, 5, 1, 1);
    test_example!(example_5_2, part_two, 5, 2, 1);
}
