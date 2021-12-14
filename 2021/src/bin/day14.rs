use std::collections::HashMap;

struct Input {
    template: String,
    rules: HashMap<String, char>,
}

fn parse_input(s: &str) -> Input {
    let mut parts = s.split("\n\n");
    let template = parts.next().unwrap().to_string();
    let rules: HashMap<_, _> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(" -> ");
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect();

    Input { template, rules }
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let mut polymer = input.template;
    println!("Template:     {}", polymer);

    for step in 1..=10 {
        let mut i = 1;
        while i < polymer.len() {
            let pair = &polymer[i - 1..=i];

            if let Some(&element) = input.rules.get(pair) {
                polymer.insert(i, element);
                i += 1
            }

            i += 1;
        }
        println!("After step {}: {}", step, polymer);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars() {
        *counts.entry(c).or_default() += 1;
    }

    let min = counts.iter().min_by_key(|(_, &count)| count).unwrap().1;
    let max = counts.iter().max_by_key(|(_, &count)| count).unwrap().1;
    let output = max - min;

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let _input = parse_input(s);

    let output = 0;

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day14.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_14_1, part_one, 14, 1, 1);
    test_example!(example_14_2, part_two, 14, 2, 1);
}
