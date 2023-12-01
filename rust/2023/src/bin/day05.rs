use std::collections::VecDeque;

use anyhow::Result;
use itertools::Itertools;

type Type = u64;

#[derive(Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

#[derive(Debug, Clone)]
struct Conversion {
    source: Range,
    destination: Range,
}

impl Conversion {
    fn offset(&self) -> i64 {
        self.destination.start as i64 - self.source.start as i64
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: Type,
    end: Type,
}

impl Range {
    fn new(start: Type, end: Type) -> Self {
        Range { start, end }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Type>,
    maps: Vec<Map>,
}

fn parse_map(s: &str) -> Map {
    let conversions = s
        .lines()
        .skip(1)
        .map(|l| {
            let mut nums = l.split(' ').map(|x| x.parse().expect("invalid number"));
            let destination_start = nums.next().expect("missing destination start");
            let source_start = nums.next().expect("missing source start");
            let length = nums.next().expect("missing length");

            Conversion {
                destination: Range::new(destination_start, destination_start + length),
                source: Range::new(source_start, source_start + length),
            }
        })
        .sorted_by(|a, b| a.source.start.cmp(&b.source.start))
        .collect();

    Map { conversions }
}

fn parse_input(s: &str) -> Almanac {
    let mut sections = s.split("\n\n");

    let seeds = sections
        .next()
        .expect("missing seeds section")
        .split_once(": ")
        .expect("invalid seeds")
        .1
        .split(' ')
        .map(|x| x.parse().expect("invalid seed"))
        .collect();

    Almanac {
        seeds,
        maps: sections.map(parse_map).collect(),
    }
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);

    input
        .seeds
        .into_iter()
        .map(|seed| {
            input.maps.iter().fold(seed, |source, map| {
                for c in &map.conversions {
                    if c.source.start <= source && source <= c.source.end {
                        return (source as i64 + c.offset()) as u64;
                    }
                }

                source
            })
        })
        .min()
        .expect("unable to find lowest location")
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);

    let ranges = input
        .seeds
        .into_iter()
        .tuples()
        .map(|(seed, len)| Range::new(seed, seed + len))
        .sorted_by(|a, b| a.start.cmp(&b.start))
        .collect::<Vec<_>>();

    input
        .maps
        .into_iter()
        .fold(ranges, |ranges, map| {
            let mut new_ranges = Vec::with_capacity(ranges.len());
            let mut queue = VecDeque::from(ranges);

            while let Some(r) = queue.pop_front() {
                let mut found = false;
                for c in &map.conversions {
                    if r.end < c.source.start || r.start > c.source.end - 1 {
                        continue;
                    }

                    found = true;

                    if r.start < c.source.start {
                        new_ranges.push(Range::new(r.start, c.source.start - 1));
                    }

                    let start = r.start.max(c.source.start) as i64 + c.offset();
                    let end = r.end.min(c.source.end) as i64 + c.offset();
                    new_ranges.push(Range::new(start as u64, end as u64));

                    // FIXME: This breaks the example input. Apparently the remainder should be
                    // tried against other conversions instead of breaking.
                    if r.end > c.source.end {
                        queue.push_front(Range::new(c.source.end, r.end + 1));
                    }

                    break;
                }

                if !found {
                    new_ranges.push(r);
                }
            }

            new_ranges.sort_by(|a, b| a.start.cmp(&b.start));
            new_ranges
        })
        .first()
        .unwrap()
        .start
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day05.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day05 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 5, 1, 1);
    test_example!(example_2_1, part_two, 5, 2, 1);
}
