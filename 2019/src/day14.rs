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

        if !self.stockpile.contains_key(&wanted.name) {
            return Err(anyhow!("unable to produce {}", wanted.name));
        }

        self.consumed
            .get(&String::from("ORE"))
            .ok_or_else(|| anyhow!("no ORE consumed"))
            .map(|&x| Chemical {
                name: "ORE".to_string(),
                amount: x,
            })
    }

    pub fn estimate(&mut self, name: &str) -> Result<usize> {
        let ore = String::from("ORE");
        let initial_ore = *self.stockpile.get(&ore).unwrap_or(&0);
        let chemical = Chemical {
            name: name.to_owned(),
            amount: 1,
        };

        let mut amount = 0;
        loop {
            let _ = self.produce(&chemical)?;

            amount += 1;

            // If we only have ORE and `name` left, we have a cycle
            if self.stockpile.len() == 2 {
                break;
            }
        }
        let ore_per_cycle = *self
            .consumed
            .get(&ore)
            .ok_or_else(|| anyhow!("no ORE consumed"))?;

        // Skip a few
        self.clear();
        let cycles = initial_ore / ore_per_cycle;
        let amount_per_cycle = amount;
        amount = cycles * amount_per_cycle;
        self.stockpile.insert(name.to_owned(), amount);
        let consumed_ore = self.consumed.entry(ore.clone()).or_insert(0);
        *consumed_ore = ore_per_cycle * cycles;
        let piled_ore = self.stockpile.entry(ore.clone()).or_insert(0);
        *piled_ore = initial_ore - *consumed_ore;

        assert!(*consumed_ore < initial_ore);
        assert!(ore_per_cycle > *piled_ore);
        assert!(*piled_ore + *consumed_ore == initial_ore);

        // Produce until we run out of supplies
        while let Ok(_) = self.produce(&chemical) {
            amount += 1;
        }

        Ok(amount)
    }

    pub fn clear(&mut self) {
        self.stockpile.clear();
        self.consumed.clear();
    }

    fn react(&mut self, reaction: &Reaction) -> Result<()> {
        if !self.filter_missing_inputs(reaction).is_empty() {
            return Err(anyhow!("missing inputs"));
        }

        for i in &reaction.inputs {
            let entry = self.stockpile.entry(i.name.clone()).or_insert(0);
            *entry -= i.amount;

            if *entry == 0 {
                self.stockpile.remove(&i.name);
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
            .filter(|i| *self.stockpile.get(&i.name).unwrap_or(&0) < i.amount)
            .collect()
    }

    fn find_reaction_for(&self, output: &Chemical) -> Option<&Reaction> {
        self.reactions.iter().find(|r| r.output.name == output.name)
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {
    input
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
        .collect()
}

#[aoc(day14, part1)]
fn answer_1(input: &[Reaction]) -> Result<usize> {
    let mut factory = Nanofactory::new(input.to_owned());
    factory.supply(&Chemical {
        name: "ORE".to_owned(),
        amount: 999_999_999,
    });

    let fuel = Chemical {
        name: "FUEL".to_string(),
        amount: 1,
    };

    factory.produce(&fuel).map(|c| c.amount)
}

#[aoc(day14, part2)]
fn answer_2(input: &[Reaction]) -> Result<usize> {
    let mut factory = Nanofactory::new(input.to_owned());
    factory.supply(&Chemical {
        name: "ORE".to_owned(),
        amount: 1_000_000_000_000,
    });

    factory.estimate("FUEL")
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

    #[test]
    fn example_2_1() {
        assert_eq!(
            82892753,
            answer_2(&input_generator(
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

    #[test]
    fn example_2_2() {
        assert_eq!(
            5586022,
            answer_2(&input_generator(
                r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                17 NVRVD, 3 JNWZP => 8 VPVL
                53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                22 VJHF, 37 MNCFX => 5 FWMGM
                139 ORE => 4 NVRVD
                144 ORE => 7 JNWZP
                5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                145 ORE => 6 MNCFX
                1 NVRVD => 8 CXFTF
                1 VJHF, 6 MNCFX => 4 RFSQX
                176 ORE => 6 VJHF"#
            ))
            .unwrap()
        );
    }

    #[test]
    fn example_2_3() {
        assert_eq!(
            460664,
            answer_2(&input_generator(
                r#"171 ORE => 8 CNZTR
                7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                114 ORE => 4 BHXH
                14 VRPVC => 6 BMBT
                6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                5 BMBT => 4 WPTQ
                189 ORE => 9 KTJDG
                1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                12 VRPVC, 27 CNZTR => 2 XDBXC
                15 KTJDG, 12 BHXH => 5 XCVML
                3 BHXH, 2 VRPVC => 7 MZWV
                121 ORE => 7 VRPVC
                7 XCVML => 6 RJRHP
                5 BHXH, 4 VRPVC => 5 LTCX"#
            ))
            .unwrap()
        );
    }
}
