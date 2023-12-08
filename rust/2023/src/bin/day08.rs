use std::{collections::HashMap, unreachable};

use anyhow::Result;

type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_input(s: &str) -> (&str, Network) {
    let (sequence, network) = s.split_once("\n\n").unwrap();

    let n = network
        .lines()
        .map(|l| {
            let (a, n) = l.split_once(" = ").unwrap();
            let (b, c) = n
                .trim_matches(&['(', ')'] as &[_])
                .split_once(", ")
                .unwrap();
            (a, (b, c))
        })
        .collect();

    (sequence, n)
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);

    let mut seq = input.0.chars().cycle();
    let mut count = 0;
    let mut place = "AAA";

    loop {
        let nodes = input.1.get(place).unwrap();
        place = match seq.next().unwrap() {
            'L' => nodes.0,
            'R' => nodes.1,
            _ => unreachable!(),
        };
        count += 1;
        if place == "ZZZ" {
            break;
        }
    }

    count.to_string()
}

fn get_cycle<'a>(mut place: &'a str, seq: &[char], network: &'a Network) -> usize {
    let mut step = 0;

    while !place.ends_with('Z') {
        let dir = seq[step % seq.len()];

        let nodes = network.get(place).unwrap();
        place = match dir {
            'L' => nodes.0,
            'R' => nodes.1,
            _ => unreachable!(),
        };

        step += 1;
    }

    step
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let seq: Vec<char> = input.0.chars().collect();

    input
        .1
        .keys()
        .filter(|&k| k.ends_with('A'))
        .map(|&k| get_cycle(k, &seq, &input.1))
        .reduce(num::integer::lcm)
        .unwrap()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day08.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day08 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 8, 1, 1);
    test_example!(example_1_2, part_one, 8, 1, 2);
    test_example!(example_2_1, part_two, 8, 2, 1);
}
