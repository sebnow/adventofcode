use anyhow::{Result, anyhow};

fn parse_input(s: &str) -> Result<Vec<u32>> {
    s.lines()
        .map(|l| l.parse().map_err(|e| anyhow!("failed to parse {}", e)))
        .collect()
}

fn part_one(s: String) -> String {
    let input = parse_input(&s).unwrap();
    for a in &input {
        for b in &input {
            if a + b == 2020 {
                return format!("{}", a * b);
            }
        }
    }

    "oops".into()
}

fn part_two(s: String) -> String {
    let input = parse_input(&s).unwrap();
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

aocutil::aoc!(2020, 01, part_one, part_two);
