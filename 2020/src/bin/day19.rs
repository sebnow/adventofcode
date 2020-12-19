use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone)]
struct Cursor<'a> {
    s: &'a [u8],
    idx: usize,
}

impl<'a> Cursor<'a> {
    fn new(s: &'a [u8], idx: usize) -> Self {
        Self { s, idx }
    }

    fn peek(&self) -> u8 {
        self.s[self.idx]
    }

    fn next(&mut self) -> u8 {
        let c = self.peek();
        self.idx += 1;
        c
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Rule {
    Char(u8),
    Seq(Vec<Box<Rule>>),
    Alt(Box<Rule>, Box<Rule>),
}

impl Rule {
    pub fn matches(&self, s: &[u8]) -> bool {
        let mut cur = Cursor::new(s, 0);
        self.matches_st(&mut cur) && cur.idx == cur.s.len()
    }

    fn matches_st(&self, mut cur: &mut Cursor) -> bool {
        let m = match self {
            Rule::Char(r) => cur.next() == *r,
            Rule::Seq(rs) => rs.iter().all(|r| r.matches_st(&mut cur)),
            Rule::Alt(a, b) => {
                let mut sub_cur = cur.clone();
                if a.matches_st(&mut sub_cur) {
                    *cur = sub_cur;
                    true
                } else {
                    let mut sub_cur = cur.clone();
                    if b.matches_st(&mut sub_cur) {
                        *cur = sub_cur;
                        true
                    } else {
                        false
                    }
                }
            }
        };

        m
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Rule::*;
        match self {
            Char(c) => write!(f, "{}", *c as char),
            Alt(a, b) => write!(f, "({}|{})", a, b),
            Seq(rs) => {
                for r in rs {
                    write!(f, "{}", r)?;
                }
                Ok(())
            }
        }
    }
}

fn maybe_alt(l: Option<Rule>, r: Option<Rule>) -> Option<Rule> {
    Some(Rule::Alt(Box::new(l?), Box::new(r?)))
}

fn parse_rule(s: &str, m: &HashMap<i64, Rule>) -> Option<Rule> {
    if s.contains('|') {
        let mut parts = s.split(" | ");
        let left = parts.next().map(|l| parse_rule(l, m)).flatten();
        let right = parts.next().map(|r| parse_rule(r, m)).flatten();
        return maybe_alt(left, right);
    }

    let rules: Option<Vec<Rule>> = s
        .split(" ")
        .map(|part| match part.chars().nth(0).unwrap() {
            '"' => Some(Rule::Char(part.bytes().nth(1).expect("missing char"))),
            _ => m
                .get(&part.parse().expect(&format!("invalid id '{}'", part)))
                .map(|r| r.clone()),
        })
        .collect();

    rules.map(|rs| {
        if rs.len() == 1 {
            return rs[0].clone();
        } else {
            return Rule::Seq(rs.iter().map(|r| Box::new(r.clone())).collect());
        }
    })
}

fn parse_input<'a>(input: &'a str) -> (Rule, impl Iterator<Item = &'a str> + 'a) {
    let mut parts = input.split("\n\n");
    let mut rule_map: HashMap<i64, Rule> = HashMap::new();
    let mut rule_queue: VecDeque<&str> = parts.next().unwrap().lines().collect();

    while let Some(s) = rule_queue.pop_front() {
        let mut rule_parts = s.split(": ");
        let rule_id = rule_parts
            .next()
            .unwrap()
            .parse()
            .expect("rule id is not a number");

        if let Some(r) = parse_rule(rule_parts.next().unwrap(), &rule_map) {
            rule_map.insert(rule_id, r);
        } else {
            rule_queue.push_back(s);
        }
    }

    (
        rule_map.get(&0).unwrap().to_owned(),
        parts.next().unwrap().lines(),
    )
}

fn part_one(input: &str) -> String {
    let (rules, messages) = parse_input(input);

    messages
        .filter(|m| rules.matches(m.as_bytes()))
        .count()
        .to_string()
}

fn part_two(input: &str) -> String {
    "".to_string()
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

    test_example!(example_one_1, part_one, 19, 1, 1);
    test_example!(example_one_2, part_one, 19, 1, 2);
    test_example!(example_one_3, part_one, 19, 1, 3);
    test_example!(example_one_4, part_one, 19, 1, 4);
    test_example!(example_one_5, part_one, 19, 1, 5);
    test_example!(example_one_6, part_one, 19, 1, 6);
    //test_example!(example_two_2, part_two, 19, 2, 1);

    #[test]
    fn parse_char() {
        assert_eq!(parse_rule("\"a\"", &HashMap::new()), Some(Rule::Char(b'a')));
    }

    #[test]
    fn parse_seq() {
        let a = Rule::Char(b'a');
        let b = Rule::Char(b'b');
        let mut m = HashMap::new();
        m.insert(1, a.clone());
        m.insert(2, b.clone());

        assert_eq!(
            parse_rule("1 2", &m),
            Some(Rule::Seq(vec![Box::new(a), Box::new(b)]))
        );
    }

    #[test]
    fn parse_alt() {
        let a = Rule::Char(b'a');
        let b = Rule::Char(b'b');
        let mut m = HashMap::new();
        m.insert(1, a.clone());
        m.insert(2, b.clone());

        assert_eq!(
            parse_rule("1 | 2", &m),
            Some(Rule::Alt(Box::new(a), Box::new(b)))
        );
    }

    #[test]
    fn match_char() {
        assert!(Rule::Char(b'a').matches(&[b'a']));
    }

    #[test]
    fn match_seq() {
        assert!(
            Rule::Seq(vec![Box::new(Rule::Char(b'a')), Box::new(Rule::Char(b'b'))])
                .matches("ab".as_bytes())
        );
    }

    #[test]
    fn match_alt() {
        let rule = Rule::Alt(Box::new(Rule::Char(b'a')), Box::new(Rule::Char(b'b')));
        assert!(rule.matches("a".as_bytes()));
        assert!(rule.matches("b".as_bytes()));
    }

    #[test]
    fn match_seq_alt() {
        let alt = Rule::Alt(Box::new(Rule::Char(b'a')), Box::new(Rule::Char(b'b')));
        let rule = Rule::Seq(vec![Box::new(alt.clone()), Box::new(alt)]);
        assert!(rule.matches("aa".as_bytes()));
        assert!(rule.matches("ab".as_bytes()));
        assert!(rule.matches("ba".as_bytes()));
        assert!(rule.matches("bb".as_bytes()));
    }
}
