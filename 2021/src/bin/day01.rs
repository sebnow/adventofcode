use anyhow::{anyhow, Result};

fn parse_input(s: &str) -> Result<Vec<u32>> {
    s.lines()
        .map(|l| l.parse().map_err(|e| anyhow!("failed to parse {}", e)))
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let mut count = 0;
    let mut prev = input[0];
    for &a in input.iter().skip(1) {
        if a > prev {
            count += 1;
        }

        prev = a;
    }

    format!("{}", count)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let mut sums: Vec<u32> = Vec::new();

    for i in 0..input.len() {
        let window: Vec<u32> = input.iter().skip(i).take(3).map(|&x| x).collect();
        if window.len() == 3 {
            sums.push(window.iter().sum());
        }
    }

    let mut count = 0;
    let mut prev = sums[0];
    for &a in sums.iter().skip(1) {
        if a > prev {
            count += 1;
        }

        prev = a;
    }

    format!("{}", count)
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day01.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 1, 1, 1);
    test_example!(example_2_1, part_two, 1, 2, 1);
}
