use anyhow::{anyhow, Result};
use rand::Rng;
use std::str::FromStr;

const LEFT_WIDTH: usize = 60;
const INDENT: &str = "| ";

fn debug<D: std::fmt::Display>(depth: usize, d: &D, msg: &str) {
    if true {return}
    println!(
        "{0:1$} | {2}",
        format!("{}{}", INDENT.repeat(depth), msg),
        LEFT_WIDTH,
        d
    );
}

fn graph(n: &Node) -> String {
    format!("digraph G {{\n{}\n}}", graph_node(n).1)
}

fn graph_node(n: &Node) -> (String, String) {
    let mut rng = rand::thread_rng();
    let n_id = format!("n{}", rng.gen::<usize>());

    let (left_id, left_output) = graph_leaf(&n.left);
    let (right_id, right_output) = graph_leaf(&n.right);
    let output = format!(
        "  {}[label = \"\"]\n  {} -> {}\n  {} -> {}\n{}\n{}",
        n_id, n_id, left_id, n_id, right_id, left_output, right_output,
    );

    (n_id, output)
}

fn graph_leaf(l: &Leaf) -> (String, String) {
    let mut rng = rand::thread_rng();
    match l {
        Leaf::Value(v) => {
            let v_id = format!("v{}", rng.gen::<usize>());
            let output = format!("  {}[label = \"{}\"]", v_id, v);
            (v_id, output)
        }
        Leaf::Child(n) => graph_node(n),
    }
}

type Num = i64;

#[derive(PartialEq, Clone, Debug)]
struct Node {
    left: Leaf,
    right: Leaf,
}

impl Node {
    pub fn new(left: Leaf, right: Leaf) -> Self {
        Node { left, right }
    }

    pub fn magnitude(&self) -> Num {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }

    pub fn reduce(&self) -> Option<Self> {
        let mut n = self.clone();
        n.reduce_rec(0)?;

        Some(n)
    }

    fn reduce_rec(&mut self, depth: usize) -> Option<()> {
        while self.explode_rec(depth).is_some() || self.split_rec().is_some() {
            debug(depth, &self, "Reducing");
        }
        debug(depth, &self, "Reduced");
        Some(())
    }

    fn split_rec(&mut self) -> Option<()> {
        self.left.split_rec().or_else(|| self.right.split_rec())
    }

    pub fn explode(&self, depth: usize) -> Option<Self> {
        let mut n = self.clone();
        n.explode_rec(depth).map(|(_, _)| n)
    }

    fn explode_rec(&mut self, depth: usize) -> Option<(bool, (Num, Num))> {
        debug(depth, &self, "Exploding");
        if depth >= 4 {
            if let (Leaf::Value(a), Leaf::Value(b)) = (&self.left, &self.right) {
                debug(depth, &self, "Bang!");
                // FIXME Pointless clone
                return Some((true, (*a, *b)));
            }
        }

        if let Some((did_explode, (l_rem, r_rem))) = self.left.explode_rec(depth + 1) {
            if did_explode {
                self.left = Leaf::Value(0);
            }

            let mut adj = &mut self.right;
            loop {
                match adj {
                    // We want the leftmost branches of the right side of the node
                    Leaf::Child(n) => adj = &mut n.left,
                    Leaf::Value(v) => {
                        *v += r_rem;
                        debug(depth, &self, "Added right value to right adjacent");
                        return Some((false, (l_rem, 0)));
                    }
                }
            }
        }

        if let Some((did_explode, (l_rem, r_rem))) = self.right.explode_rec(depth + 1) {
            if did_explode {
                self.right = Leaf::Value(0);
            }

            let mut adj = &mut self.left;
            loop {
                match adj {
                    // We want the rightmost branches of the left side of the node
                    Leaf::Child(n) => adj = &mut n.right,
                    Leaf::Value(v) => {
                        *v += l_rem;
                        debug(depth, &self, "Added left value to left adjacent");
                        return Some((false, (0, r_rem)));
                    }
                }
            }
        }

        None
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(
            matches!(s.chars().next(), Some('[')),
            "opening bracket not found in \"{}\"",
            s
        );
        assert!(
            matches!(s.chars().last(), Some(']')),
            "closing bracket not found in \"{}\"",
            s
        );

        let (left, right) = if &s[1..2] == "[" {
            let end = find_end_of_nesting(&s[1..])
                .ok_or_else(|| anyhow!("unable to find end of nesting in \"{}\"", &s[1..]))?;
            let left_part = &s[1..=end + 1];

            let left = Leaf::from_str(left_part)?;
            let right = Leaf::from_str(&s[left_part.len() + 2..s.len() - 1])?;

            (left, right)
        } else {
            let (left, right) = s[1..s.len() - 1]
                .split_once(',')
                .ok_or_else(|| anyhow!("expected pair in \"{}\"", s))?;
            (Leaf::from_str(left)?, Leaf::from_str(right)?)
        };

        Ok(Node { left, right })
    }
}

impl std::ops::Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        let n = Node::new(Leaf::Child(Box::new(self)), Leaf::Child(Box::new(rhs)));
        n.reduce().unwrap_or(n)
    }
}

impl std::iter::Sum for Node {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap()
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Leaf {
    Child(Box<Node>),
    Value(Num),
}

impl Leaf {
    pub fn magnitude(&self) -> Num {
        match self {
            Leaf::Value(v) => *v,
            Leaf::Child(n) => n.magnitude(),
        }
    }

    pub fn split(&self) -> Option<Self> {
        let mut l = self.clone();
        l.split_rec().map(|_| l)
    }

    fn split_rec(&mut self) -> Option<()> {
        match self {
            Leaf::Value(v) if *v >= 10 => {
                *self = Leaf::Child(Box::new(Node::new(
                    Leaf::Value(*v / 2),
                    Leaf::Value((*v + 1) / 2),
                )));
                Some(())
            }
            Leaf::Child(n) => n.split_rec(),
            _ => None,
        }
    }

    fn explode_rec(&mut self, depth: usize) -> Option<(bool, (Num, Num))> {
        match self {
            Leaf::Value(_) => None,
            Leaf::Child(n) => n.explode_rec(depth).map(|(is_first, rem)| (is_first, rem)),
        }
    }
}

impl FromStr for Leaf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..1] == "[" {
            Ok(Leaf::Child(Box::new(Node::from_str(s)?)))
        } else {
            let v = s.parse()?;
            Ok(Leaf::Value(v))
        }
    }
}

impl std::fmt::Display for Leaf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Leaf::Value(v) => v.fmt(f),
            Leaf::Child(c) => c.fmt(f),
        }
    }
}

fn find_end_of_nesting(s: &str) -> Option<usize> {
    assert_eq!(&s[0..1], "[");

    let mut nesting = 0;
    for (i, c) in s.chars().enumerate() {
        nesting += match c {
            '[' => 1,
            ']' => -1,
            _ => 0,
        };
        if nesting == 0 {
            return Some(i);
        }
    }

    None
}

fn parse_input(s: &str) -> impl Iterator<Item = Node> + '_ {
    s.lines().map(|l| l.parse().unwrap())
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);

    let output = input.sum::<Node>().magnitude();
    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input: Vec<Node> = parse_input(s).collect();

    let mut output = 0;
    for a in &input {
        for b in &input {
            if a == b {
                continue
            }

            output = output.max((a.to_owned() + b.to_owned()).magnitude());
        }
    }

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day18.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day18 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_18_1_1, part_one, 18, 1, 1);
    test_example!(example_18_1_2, part_one, 18, 1, 2);
    test_example!(example_18_1_3, part_one, 18, 1, 3);
    test_example!(example_18_1_4, part_one, 18, 1, 4);
    test_example!(example_18_1_5, part_one, 18, 1, 5);
    test_example!(example_18_1_6, part_one, 18, 1, 6);
    test_example!(example_18_1_7, part_one, 18, 1, 7);
    test_example!(example_18_1_8, part_one, 18, 1, 8);
    test_example!(example_18_1_9, part_one, 18, 1, 9);
    test_example!(example_18_1_10, part_one, 18, 1, 10);
    test_example!(example_18_2_1, part_two, 18, 2, 1);

    #[test]
    fn parse_leaf_value() {
        assert_eq!(Leaf::from_str("42").unwrap(), Leaf::Value(42));
    }

    #[test]
    fn parse_leaf_child() {
        assert_eq!(
            Leaf::from_str("[4,2]").unwrap(),
            Leaf::Child(Box::new(Node::new(Leaf::Value(4), Leaf::Value(2))))
        );
    }

    #[test]
    fn parse_node_with_values() {
        assert_eq!(
            Node::from_str("[4,2]").unwrap(),
            Node::new(Leaf::Value(4), Leaf::Value(2)),
        );
    }

    #[test]
    fn parse_node_with_children() {
        assert_eq!(
            Node::from_str("[[1,2],3]").unwrap(),
            Node::new(
                Leaf::Child(Box::new(Node::new(Leaf::Value(1), Leaf::Value(2)))),
                Leaf::Value(3)
            ),
        );

        assert_eq!(
            Node::from_str("[1,[2,3]]").unwrap(),
            Node::new(
                Leaf::Value(1),
                Leaf::Child(Box::new(Node::new(Leaf::Value(2), Leaf::Value(3)))),
            ),
        );
        assert_eq!(
            Node::from_str("[[12,13],[14,15]]").unwrap(),
            Node::new(
                Leaf::Child(Box::new(Node::new(Leaf::Value(12), Leaf::Value(13)))),
                Leaf::Child(Box::new(Node::new(Leaf::Value(14), Leaf::Value(15)))),
            ),
        );
    }

    #[test]
    fn symmetric_display_from_str() {
        let str = "[1,[[2,3],4]]";
        assert_eq!(Node::from_str(str).unwrap().to_string(), str);
    }

    #[test]
    fn split() {
        let assert_split = |value, expected| {
            assert_eq!(
                Leaf::from_str(value).unwrap().split().unwrap().to_string(),
                expected
            )
        };

        assert_split("10", "[5,5]");
        assert_split("11", "[5,6]");
        assert_split("12", "[6,6]");
    }

    #[test]
    fn add() {
        let assert_added = |a, b, expected| {
            assert_eq!(
                (Node::from_str(a).unwrap() + Node::from_str(b).unwrap()).to_string(),
                expected
            );
        };

        assert_added(
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    #[test]
    fn explode() {
        let assert_exploded = |node, expected| {
            let n = Node::from_str(node).unwrap();
            println!("{:width$} => {}", node, expected, width = LEFT_WIDTH);
            assert_eq!(
                n.explode(0).unwrap().to_string(),
                expected,
                "\n{}",
                graph(&n)
            );
            println!("\n");
        };

        assert_exploded("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"); // Explode
        assert_exploded("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"); // Explode
        assert_exploded("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"); // Explode
        assert_exploded(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ); // Explode
        assert_exploded(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ); // Explode
    }

    #[test]
    fn reduce() {
        let assert_reduced = |node, expected| {
            let n = Node::from_str(node).unwrap();
            println!("{:width$} => {}", node, expected, width = LEFT_WIDTH);
            assert_eq!(n.reduce().unwrap().to_string(), expected, "\n{}", graph(&n));
            println!("\n");
        };

        assert_reduced("[15,[0,1]]", "[[7,8],[0,1]]"); // Split
        assert_reduced("[[7,8],[0,13]]", "[[7,8],[0,[6,7]]]"); // Split
        assert_reduced("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"); // Explode
        assert_reduced("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"); // Explode
        assert_reduced("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"); // Explode
        assert_reduced(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ); // Explode
        assert_reduced(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ); // Total
    }
}
