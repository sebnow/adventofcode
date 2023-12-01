use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn in_range(v: i64, r: &Range) -> bool {
    r.0 <= v && v <= r.1
}

fn get_invalid<'a>(rules: &Vec<Rule>, t: &'a Ticket) -> Vec<&'a i64> {
    t.iter()
        .filter(|&v| {
            !rules
                .iter()
                .any(|r| r.ranges.iter().any(|r| in_range(*v, r)))
        })
        .collect()
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
        .flat_map(|t| get_invalid(&spec.rules, t))
        .sum::<i64>()
        .to_string()
}

fn part_two(input: &str) -> String {
    let spec = parse_input(input);

    assert!(spec.nearby[0].len() == spec.rules.len());

    let nearby: Vec<_> = spec
        .nearby
        .iter()
        .filter(|t| get_invalid(&spec.rules, t).is_empty())
        .collect();

    let mut possible: Vec<Vec<String>> = std::iter::repeat(Vec::new())
        .take(spec.rules.len())
        .collect();

    for idx in 0..nearby[0].len() {
        spec.rules
            .iter()
            .filter(|rule| {
                nearby
                    .iter()
                    .map(|n| n[idx])
                    .all(|v| rule.ranges.iter().any(|range| in_range(v, range)))
            })
            .for_each(|rule| possible[idx].push(rule.field.clone()));
    }

    let mut inverted: HashMap<&str, Vec<usize>> = HashMap::new();
    for (idx, fields) in possible.iter().enumerate() {
        for field in fields {
            inverted.entry(field).or_default().push(idx);
        }
    }

    let mut queue: VecDeque<(&str, Vec<usize>)> = inverted.iter().map(|(f, i)| (*f, i.clone())).collect();
    let mut seen = HashSet::new();
    let mut field_map: HashMap<&str, usize> = HashMap::new();
    while let Some((field, indices)) = queue.pop_front() {
        if indices.len() > 1 {
            queue.push_back((
                field,
                indices
                    .iter()
                    .filter(|&i| !seen.contains(i))
                    .map(|&i| i)
                    .collect(),
            ));
            continue;
        }

        let index = indices[0];
        seen.insert(index);
        field_map.insert(field, index);
    }

    field_map
        .iter()
        .filter_map(|(field, index)| {
            if field.starts_with("departure") {
                Some(spec.ticket[*index])
            } else {
                None
            }
        })
        .product::<i64>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day16.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 16, 1, 1);
    test_example!(example_two_1, part_two, 16, 2, 1);
}
