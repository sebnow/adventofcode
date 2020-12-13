use anyhow::{anyhow, Result};

enum Bus {
    ID(i64),
    OutOfService,
}

impl std::str::FromStr for Bus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Bus::OutOfService),
            _ => Ok(Bus::ID(s.parse()?)),
        }
    }
}

fn parse_input(s: &str) -> Result<(i64, Vec<Bus>)> {
    let mut lines = s.lines();
    let est = lines
        .next()
        .ok_or_else(|| anyhow!("missing estimate"))?
        .parse()?;
    let buses = lines
        .next()
        .ok_or_else(|| anyhow!("missing busses"))?
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<Vec<Bus>>>()?;

    Ok((est, buses))
}

fn part_one(input: &str) -> String {
    let (estimate, buses) = parse_input(input).unwrap();
    let mut time = estimate;

    loop {
        for bus in &buses {
            match bus {
                Bus::ID(id) => {
                    if time % id == 0 {
                        return ((time - estimate) * id).to_string();
                    }
                }
                Bus::OutOfService => continue,
            }
        }

        time += 1;
    }
}

fn part_two(input: &str) -> String {
    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day13.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 13, 1, 1);
}
