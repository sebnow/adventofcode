use anyhow::Result;

#[derive(Debug)]
struct Range(i64, i64);

#[derive(Debug)]
struct Rule {
    field: String,
    ranges: Vec<Range>,
}

type Ticket = Vec<i64>;

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby: Vec<Ticket>,
}

fn parse_input(s: &str) -> Input {
    let mut parts = s.trim().split("\n\n");

    Input {
        rules: parts
            .next()
            .expect("missing rules")
            .lines()
            .map(|l| {
                let mut parts = l.split(": ");
                let field = parts.next().expect("missing field").to_string();
                let ranges = parts
                    .next()
                    .expect("missing ranges")
                    .split(" or ")
                    .map(|r| {
                        let mut values = r
                            .split("-")
                            .map(|n| n.parse::<i64>().expect("invalid range"));
                        Range(
                            values.next().expect("missing lower bound"),
                            values.next().expect("missing upper bound"),
                        )
                    })
                    .collect();

                Rule { field, ranges }
            })
            .collect(),
        ticket: parts
            .next()
            .expect("missing ticket")
            .split(":\n")
            .nth(1)
            .expect("missing ticket")
            .split(',')
            .map(|n| n.parse().expect("invalid value"))
            .collect(),
        nearby: parts
            .next()
            .expect("missing nearby")
            .split(":\n")
            .nth(1)
            .expect("missing nearby")
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|n| n.parse::<i64>().expect("invalid value"))
                    .collect()
            })
            .collect(),
    }
}

fn part_one(input: &str) -> String {
    let spec = parse_input(input);

    spec.nearby
        .iter()
        .flatten()
        .filter(|&v| {
            !spec.rules.iter().any(|r| {
                r.ranges
                    .iter()
                    .any(|Range(min, max)| *min <= *v && *v <= *max)
            })
        })
        .sum::<i64>()
        .to_string()
}

fn part_two(input: &str) -> String {
    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day16.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 16, 1, 1);
    //test_example!(example_two_1, part_two, 14, 2, 1);
}
