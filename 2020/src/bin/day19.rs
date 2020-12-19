use anyhow::{anyhow, Result};
use std::collections::HashMap;
use rayon::str::ParallelString;
use rayon::iter::ParallelIterator;

#[derive(Clone, PartialEq, Debug)]
enum Rule {
    Char(char),
    Seq(Vec<i64>),
    Alt(Vec<i64>, Vec<i64>),
}

impl std::str::FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('|') {
            let mut parts = s.split(" | ");
            let left = parts
                .next()
                .ok_or_else(|| anyhow!("missing left alternate"))?
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect();
            let right = parts
                .next()
                .ok_or_else(|| anyhow!("missing right alternate"))?
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect();

            Ok(Rule::Alt(left, right))
        } else if s.starts_with('"') {
            let c = s
                .chars()
                .nth(1)
                .ok_or_else(|| anyhow!("missing reference"))?;

            Ok(Rule::Char(c))
        } else {
            let ids = s
                .split(' ')
                .map(|id| {
                    id.parse()
                        .map_err(|err| anyhow!("invalid reference: {}", err))
                })
                .collect::<Result<_>>()?;

            Ok(Rule::Seq(ids))
        }
    }
}

struct RuleEngine {
    rules: HashMap<i64, Rule>,
}

impl RuleEngine {
    pub fn new(rules: HashMap<i64, Rule>) -> Self {
        RuleEngine { rules }
    }

    pub fn matches(&self, s: &str) -> bool {
        self.match_rule_id(s, 0).contains(&Some(""))
    }

    fn match_rule_id<'a>(&self, s: &'a str, rule_id: i64) -> Vec<Option<&'a str>> {
        let rule = self.rules.get(&rule_id).unwrap();

        self.match_rule(s, rule)
    }

    fn match_rule<'a>(&self, s: &'a str, rule: &Rule) -> Vec<Option<&'a str>> {
        match rule {
            Rule::Char(c) if s.chars().next() == Some(*c) => vec![Some(&s[1..])],
            Rule::Char(_) => vec![None],
            Rule::Seq(rs) => self.match_seq(s, rs),
            Rule::Alt(left, right) => self.match_alt(s, left, right),
        }
    }

    fn match_seq<'a>(&self, s: &'a str, rules: &[i64]) -> Vec<Option<&'a str>> {
        rules.iter().fold(vec![Some(s)], |ss, r| {
            ss.iter()
                .flat_map(|s| match s {
                    Some(s) if !s.is_empty() => self.match_rule_id(s, *r),
                    _ => vec![None],
                })
                .collect()
        })
    }

    fn match_alt<'a>(&self, s: &'a str, left: &[i64], right: &[i64]) -> Vec<Option<&'a str>> {
        [left, right]
            .iter()
            .flat_map(|rs| self.match_seq(s, rs))
            .collect()
    }
}

impl std::str::FromStr for RuleEngine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .par_lines()
            .map(|l| {
                let mut parts = l.split(": ");
                Ok((
                    parts.next().ok_or_else(|| anyhow!("missing id"))?.parse()?,
                    parts
                        .next()
                        .ok_or_else(|| anyhow!("missing definition"))?
                        .parse()?,
                ))
            })
            .collect::<Result<_>>()?;

        Ok(RuleEngine::new(rules))
    }
}

fn parse_input<'a>(input: &'a str) -> (RuleEngine, impl ParallelIterator<Item = &'a str> + 'a) {
    let mut parts = input.split("\n\n");
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().par_lines(),
    )
}

fn part_one(input: &str) -> String {
    let (rules, messages) = parse_input(input);

    messages.filter(|m| rules.matches(m)).count().to_string()
}

fn part_two(input: &str) -> String {
    let input: String = input
        .lines()
        .map(|l| {
            if l.starts_with("8: ") {
                "8: 42 | 42 8"
            } else if l.starts_with("11: ") {
                "11: 42 31 | 42 11 31"
            } else {
                l
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");

    part_one(&input)
}

fn main() {
    let input = include_str!("../../input/day19.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;
    use std::str::FromStr;

    test_example!(example_one_1, part_one, 19, 1, 1);
    test_example!(example_one_2, part_one, 19, 1, 2);
    test_example!(example_one_3, part_one, 19, 1, 3);
    test_example!(example_one_4, part_one, 19, 1, 4);
    test_example!(example_one_5, part_one, 19, 1, 5);
    test_example!(example_one_6, part_one, 19, 1, 6);
    test_example!(example_two_1, part_two, 19, 2, 1);
    test_example!(example_two_2, part_two, 19, 2, 2);
    test_example!(example_two_3, part_two, 19, 2, 3);
    test_example!(example_two_4, part_two, 19, 2, 4);
    test_example!(example_two_5, part_two, 19, 2, 5);

    #[test]
    fn parse_char() -> Result<()> {
        assert_eq!(Rule::from_str("\"a\"")?, Rule::Char('a'));
        Ok(())
    }

    #[test]
    fn parse_seq() -> Result<()> {
        assert_eq!(Rule::from_str("1 2")?, Rule::Seq(vec![1, 2]));
        Ok(())
    }

    #[test]
    fn parse_alt() -> Result<()> {
        assert_eq!(Rule::from_str("1 | 2")?, alt(vec![1], vec![2]),);
        Ok(())
    }

    #[test]
    fn match_char() {
        let mut rules = HashMap::new();
        rules.insert(0, Rule::Char('a'));

        assert!(RuleEngine::new(rules).matches("a"));
    }

    #[test]
    fn match_seq() {
        let mut rules = HashMap::new();
        rules.insert(0, Rule::Seq(vec![1, 2]));
        rules.insert(1, Rule::Char('a'));
        rules.insert(2, Rule::Char('b'));

        assert!(RuleEngine::new(rules).matches("ab"));
    }

    #[test]
    fn match_alt() {
        let mut rules = HashMap::new();
        rules.insert(0, alt(vec![1], vec![2]));
        rules.insert(1, Rule::Char('a'));
        rules.insert(2, Rule::Char('b'));

        let engine = RuleEngine::new(rules);
        assert!(engine.matches("a"));
        assert!(engine.matches("b"));
    }

    #[test]
    fn match_seq_alt() {
        let mut rules = HashMap::new();
        rules.insert(0, Rule::Seq(vec![1, 1]));
        rules.insert(1, alt(vec![2], vec![3]));
        rules.insert(2, Rule::Char('a'));
        rules.insert(3, Rule::Char('b'));

        let engine = RuleEngine::new(rules);
        assert!(engine.matches("aa"));
        assert!(engine.matches("ab"));
        assert!(engine.matches("ba"));
        assert!(engine.matches("bb"));
    }

    fn alt(left: Vec<i64>, right: Vec<i64>) -> Rule {
        Rule::Alt(left, right)
    }
}
