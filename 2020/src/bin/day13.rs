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
    let buses: Vec<(i64, i64)> = parse_input(input).unwrap().1.iter().enumerate().filter_map(|(i, bus)| {
        match bus {
            Bus::OutOfService => None,
            Bus::ID(x) => Some((i as i64, *x)),
        }
    }).collect();

    let mut time = buses[0].1;
    loop {
        if buses.iter().all(|(offset, bus)| (time + offset) % bus == 0) {
            return time.to_string()
        }

        time += 1;
    }
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
    test_example!(example_two_1, part_two, 13, 2, 1);
    test_example!(example_two_2, part_two, 13, 2, 2);
    test_example!(example_two_3, part_two, 13, 2, 3);
    test_example!(example_two_4, part_two, 13, 2, 4);
    test_example!(example_two_5, part_two, 13, 2, 5);
    test_example!(example_two_6, part_two, 13, 2, 6);
}
