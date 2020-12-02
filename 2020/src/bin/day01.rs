use anyhow::{anyhow, Result};

fn parse_input(s: &str) -> Result<Vec<u32>> {
    s.lines()
        .map(|l| l.parse().map_err(|e| anyhow!("failed to parse {}", e)))
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    for a in &input {
        for b in &input {
            if a + b == 2020 {
                return format!("{}", a * b);
            }
        }
    }

    "oops".into()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    for a in &input {
        for b in &input {
            for c in &input {
                if a + b + c == 2020 {
                    return format!("{}", a * b * c);
                }
            }
        }
    }

    "oops".into()
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
