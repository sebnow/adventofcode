use anyhow::{anyhow, Context, Result};

fn parse_input(s: &str) -> Result<Vec<Vec<u64>>> {
    s.split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse().with_context(|| "failed to parse inventory"))
                .collect()
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let most: u64 = input
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .ok_or_else(|| anyhow!("unable to find max"))
        .unwrap();

    most.to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();

    let mut totals: Vec<u64> = input.iter().map(|elf| elf.iter().sum()).collect();

    totals.sort();

    let top3: u64 = totals.iter().rev().take(3).sum();

    top3.to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day01.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 1, 1, 1);
    test_example!(example_1_2, part_two, 1, 2, 1);
}
