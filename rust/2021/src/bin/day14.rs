use itertools::Itertools;
use std::collections::HashMap;

type Pair = (char, char);

struct Input {
    template: String,
    rules: HashMap<Pair, char>,
}

fn solve(s: &str, steps: usize) -> usize {
    let input = parse_input(s);
    let mut pairs: HashMap<Pair, usize> = HashMap::new();

    pairs.insert((input.template.chars().last().unwrap(), '\0'), 1);
    for pair in input.template.chars().tuple_windows() {
        *pairs.entry(pair).or_default() += 1;
    }

    for _ in 1..=steps {
        let ps = pairs.keys().copied().collect_vec();
        let mut new_pairs = HashMap::with_capacity(pairs.len());

        for p @ (a, b) in ps {
            let count = *pairs.entry((a, b)).or_default();
            if let Some(&c) = input.rules.get(&p) {
                *new_pairs.entry((a, c)).or_default() += count;
                *new_pairs.entry((c, b)).or_default() += count;
            } else {
                *new_pairs.entry((a, b)).or_default() += count;
            };
        }

        pairs = new_pairs;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for (&(a, _), count) in &pairs {
        *counts.entry(a).or_default() += count;
    }
    let (min, max) = counts.into_values().minmax().into_option().unwrap();

    max - min
}

fn parse_input(s: &str) -> Input {
    let mut parts = s.split("\n\n");
    let template = parts.next().unwrap().to_string();
    let rules: HashMap<_, _> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut cs = l.chars();
            ((cs.next().unwrap(), cs.next().unwrap()), cs.nth(4).unwrap())
        })
        .collect();

    Input { template, rules }
}

fn part_one(s: &str) -> String {
    format!("{}", solve(s, 10))
}

fn part_two(s: &str) -> String {
    format!("{}", solve(s, 40))
}

fn main() {
    let input = include_str!("../../../../input/2021/day14.txt");
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
