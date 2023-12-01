use anyhow::Result;
use std::collections::{VecDeque, HashMap, HashSet};

fn parse_input<'a>(s: &'a str) -> HashMap<String, Vec<(u64, String)>> {
    s.lines().map(|l| {
        let (l, _) = l.split_at(l.len() - 1);
        let mut parts = l.split(" bags contain ");
        let outer = parts.next().unwrap();
        let rest = parts.next().unwrap();
        let inner: Vec<(u64, String)> = rest
            .split(", ")
            .filter_map(|i| {
                if i == "no other bags" {
                    None
                } else {
                    let mut parts = i.split(" ");
                    let amount = parts.next().unwrap();
                    let tint = parts.next().unwrap();
                    let colour = parts.next().unwrap();

                    Some((amount.parse().unwrap(), format!("{} {}", tint, colour)))
                }
            })
            .collect();

        (outer.to_string(), inner)
    }).collect()
}

fn part_one(input: &str) -> String {
    let mut bags: VecDeque<String> = VecDeque::default();
    let rules = parse_input(input);

    bags.push_back("shiny gold".into());

    let mut inverted: HashMap<String, HashSet<String>> = HashMap::with_capacity(rules.len());
    for (outer, inner) in rules {
        for (_, bag) in inner {
            let entry = inverted.entry(bag.clone()).or_default();
            entry.insert(outer.clone());
        }
    }

    let mut answer: HashSet<String> = HashSet::default() ;
    while let Some(bag) = bags.pop_front() {
        if let Some(outer) = inverted.get(&bag) {
            for b in outer {
                bags.push_back(b.clone());
                answer.insert(b.clone());
            }
        }
    }

    answer.len().to_string()
}

fn part_two(input: &str) -> String {
    let rules = parse_input(input);
    let mut bags: VecDeque<String> = VecDeque::default();

    bags.push_back("shiny gold".into());

    let mut total = 0;
    while let Some(bag) = bags.pop_front() {
        let contents = rules.get(&bag).unwrap();
        for (amount, inner) in contents {
            total += amount;
            for _ in 0..*amount {
                bags.push_back(inner.clone());
            }
        }
    }

    total.to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day07.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 7, 1, 1);
    test_example!(example_two_1, part_two, 7, 2, 1);
}
