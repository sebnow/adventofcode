use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Chemical {
    amount: usize,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

fn parse_chemical(s: &str) -> Chemical {
    let mut parts = s.trim().split(' ');

    Chemical {
        amount: parts
            .next()
            .expect("missing amount")
            .parse()
            .expect("invalid amount"),
        name: parts.next().expect("missing name").to_owned(),
    }
}

struct Nanofactory {
    consumed: HashMap<String, usize>,
    stockpile: HashMap<String, usize>,
    reactions: Vec<Reaction>,
}

impl Nanofactory {
    pub fn new(reactions: Vec<Reaction>) -> Self {
        Nanofactory {
            consumed: HashMap::new(),
            stockpile: HashMap::new(),
            reactions,
        }
    }

    pub fn supply(&mut self, chemical: &Chemical) {
        let output = self.stockpile.entry(chemical.name.clone()).or_insert(0);
        *output += chemical.amount;
    }

    pub fn produce(&mut self, wanted: &Chemical) -> Result<Chemical> {
        let mut queue = VecDeque::new();
        queue.push_back(wanted.clone());

        while let Some(chemical) = queue.pop_front() {
            let reaction = self
                .find_reaction_for(&chemical)
                .ok_or_else(|| anyhow!("missing chemical {}", chemical.name))?
                .clone();
            let missing = self.filter_missing_inputs(&reaction);

            if missing.is_empty() {
                self.react(&reaction)?;
            } else {
                // Requeue the original chemical to attempt to produce it again
                queue.push_front(chemical);
                for m in missing {
                    queue.push_front(m.clone());
                }
            }
        }

        self.consumed
            .get(&String::from("ORE"))
            .ok_or_else(|| anyhow!("no ORE produced"))
            .map(|&x| Chemical {
                name: "ORE".to_string(),
                amount: x,
            })
    }

    fn react(&mut self, reaction: &Reaction) -> Result<()> {
        assert!(self.filter_missing_inputs(reaction).is_empty());

        for i in &reaction.inputs {
            if i.name != "ORE" {
                let entry = self.stockpile.entry(i.name.clone()).or_insert(0);
                *entry -= i.amount;
            }

            let consumed = self.consumed.entry(i.name.clone()).or_insert(0);
            *consumed += i.amount;
        }

        self.supply(&reaction.output);

        Ok(())
    }

    fn filter_missing_inputs<'a>(&'a self, reaction: &'a Reaction) -> Vec<&'a Chemical> {
        reaction
            .inputs
            .iter()
            .filter(move |i| {
                if i.name == "ORE" {
                    return false;
                }

                *self.stockpile.get(&i.name).unwrap_or(&0) < i.amount
            })
            .collect()
    }

    fn find_reaction_for(&self, output: &Chemical) -> Option<&Reaction> {
        self.reactions.iter().find(|r| r.output.name == output.name)
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {
    let mut reactions: Vec<Reaction> = input
        .lines()
        .map(|l| {
            let mut parts = l.trim().split("=>");
            Reaction {
                inputs: parts
                    .next()
                    .expect("missing inputs")
                    .split(", ")
                    .map(parse_chemical)
                    .collect(),
                output: parse_chemical(parts.next().expect("missing outputs")),
            }
        })
        .collect();

    let ore = Chemical {
        name: "ORE".to_owned(),
        amount: 1,
    };
    reactions.push(Reaction {
        inputs: vec![],
        output: ore,
    });
    reactions
}

#[aoc(day14, part1)]
fn answer_1(input: &[Reaction]) -> Result<usize> {
    let mut factory = Nanofactory::new(input.to_owned());
    let ore = factory.produce(&Chemical {
        name: "FUEL".to_string(),
        amount: 1,
    })?;

    Ok(ore.amount)
}

#[aoc(day14, part2)]
fn answer_2(input: &[Reaction]) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            18,
            answer_1(&input_generator(
                r#"9 ORE => 2 A
                3 A => 1 FUEL"#
            ))
            .unwrap()
        );

        assert_eq!(
            165,
            answer_1(&input_generator(
                r#"9 ORE => 2 A
                8 ORE => 3 B
                7 ORE => 5 C
                3 A, 4 B => 1 AB
                5 B, 7 C => 1 BC
                4 C, 1 A => 1 CA
                2 AB, 3 BC, 4 CA => 1 FUEL"#
            ))
            .unwrap()
        );
        assert_eq!(
            13312,
            answer_1(&input_generator(
                r#"157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#
            ))
            .unwrap()
        );
    }
}
