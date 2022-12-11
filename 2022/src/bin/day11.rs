use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Context, Result};

type WorryLevel = i64;
type Operation = dyn Fn(WorryLevel) -> WorryLevel;

#[derive(Debug)]
struct Test {
    div_by: i64,
    on_true: usize,
    on_false: usize,
}

struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Box<Operation>,
    test: Test,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("test", &self.test)
            .finish()
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);

        let items: VecDeque<WorryLevel> = lines
            .next()
            .and_then(|l| l.split_once("items: "))
            .ok_or_else(|| anyhow!("missing items"))
            .and_then(|(_, nums)| {
                nums.split(", ")
                    .map(|n| n.parse().with_context(|| "parsing item"))
                    .collect()
            })?;

        let operation = lines
            .next()
            .and_then(|l| l.split_once(" = "))
            .ok_or_else(|| anyhow!("failed to parse operation"))
            .and_then(|(_, str)| -> Result<Box<Operation>> {
                let mut parts = str.split(' ');

                let lhs = parts
                    .next()
                    .ok_or_else(|| anyhow!("missing operation lhs"))?;
                let op = parts
                    .next()
                    .ok_or_else(|| anyhow!("missing operation operand"))?;
                let rhs = parts
                    .next()
                    .ok_or_else(|| anyhow!("missing operation rhs"))?;

                Ok(match (lhs, op, rhs) {
                    ("old", "*", "old") => Box::new(|n| n * n),
                    ("old", "*", rhs) => {
                        let x: i64 = rhs.parse()?;
                        Box::new(move |n| n * x)
                    }
                    ("old", "+", rhs) => {
                        let x: i64 = rhs.parse()?;
                        Box::new(move |n| n + x)
                    }
                    _ => return Err(anyhow!("unsupported operation: {}", str)),
                })
            })?;

        let (_, divisible_by) = lines
            .next()
            .and_then(|l| l.split_once("divisible by "))
            .with_context(|| "parsing test condition")?;

        let on_true = lines
            .next()
            .and_then(|l| l.split_once("true: throw to monkey "))
            .ok_or_else(|| anyhow!("missing true case"))
            .and_then(|(_, m)| m.parse::<usize>().with_context(|| "parsing monkey number"))?;

        let on_false = lines
            .next()
            .and_then(|l| l.split_once("false: throw to monkey "))
            .ok_or_else(|| anyhow!("missing false case"))
            .and_then(|(_, m)| m.parse::<usize>().with_context(|| "parsing monkey number"))?;

        Ok(Monkey {
            items,
            operation,
            test: Test {
                div_by: divisible_by.parse()?,
                on_true,
                on_false,
            },
        })
    }
}

fn parse_input(s: &str) -> Result<Vec<Monkey>> {
    s.split("\n\n").map(Monkey::from_str).collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let mut monkeys = input;
    let mut activity: Vec<usize> = std::iter::repeat(0).take(monkeys.len()).collect();

    for _ in 0..20 {
        for m_idx in 0..monkeys.len() {
            while let Some(worry_level) = monkeys[m_idx].items.pop_front() {
                activity[m_idx] += 1;
                let mut new_level = (monkeys[m_idx].operation)(worry_level);
                new_level /= 3;

                let new_monkey = if new_level % monkeys[m_idx].test.div_by == 0 {
                    monkeys[m_idx].test.on_true
                } else {
                    monkeys[m_idx].test.on_false
                };

                monkeys[new_monkey].items.push_back(new_level)
            }
        }
    }

    activity.sort();
    activity.reverse();
    (activity[0] * activity[1]).to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day11.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two:\n{}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(day11_1, part_one, 11, 1, 1);
    //test_example!(day11_2, part_two, 11, 2, 1);
}
