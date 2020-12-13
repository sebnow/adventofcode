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

fn parse_input<'a>(s: &'a str) -> (i64, impl Iterator<Item = Bus> + 'a) {
    let mut lines = s.lines();
    let est = lines
        .next()
        .expect("missing estimate")
        .parse().unwrap();
    let buses = lines
        .next()
        .expect("missing busses")
        .split(",")
        .map(|x| x.parse().unwrap());

    (est, buses)
}

fn part_one(input: &str) -> String {
    let (estimate, buses) = parse_input(input);
    let mut time = estimate;
    let buses: Vec<Bus> = buses.collect();

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
    parse_input(input)
        .1
        .enumerate()
        .filter_map(|(dt, bus)| match bus {
            Bus::OutOfService => None,
            Bus::ID(id) => Some((dt as i64, id)),
        })
        .fold((0, 1), |(t, step), (dt, id)| {
            (
                (t..std::i64::MAX)
                    .step_by(step as usize)
                    .find(|t| (t + dt) % id == 0)
                    .unwrap(),
                step * id,
            )
        })
        .0
        .to_string()
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
