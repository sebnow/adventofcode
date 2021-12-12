use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

type Edges = HashMap<Cave, Vec<Cave>>;

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
enum Cave {
    Big(String),
    Small(String),
}

impl Cave {
    fn new(s: String) -> Cave {
        if s.to_lowercase() == s {
            Cave::Small(s)
        } else {
            Cave::Big(s)
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Path {
    visited: HashSet<Cave>,
    breadcrumbs: Vec<Cave>,
}

impl Path {
    fn visit(&mut self, cave: &Cave) -> Result<()> {
        if matches!(cave, Cave::Small(_)) {
            if self.visited.contains(cave) {
                return Err(anyhow!("already visited"));
            }

            self.visited.insert(cave.clone());
        }

        self.breadcrumbs.push(cave.clone());
        Ok(())
    }

    fn connections(&self, connections: &Edges) -> Option<Vec<Cave>> {
        let caves = connections.get(self.last()?)?;
        Some(
            caves
                .iter()
                .filter(|c| !self.visited.contains(c))
                .cloned()
                .collect(),
        )
    }

    fn last(&self) -> Option<&'_ Cave> {
        self.breadcrumbs.last()
    }
}

fn parse_input(s: &str) -> Edges {
    let connections = s.lines().map(|l| {
        let mut connection = l.split('-');
        (
            Cave::new(connection.next().unwrap().to_string()),
            Cave::new(connection.next().unwrap().to_string()),
        )
    });

    connections.fold(Edges::new(), |mut edges, (a, b)| {
        edges.entry(a.clone()).or_insert_with(Vec::new).push(b.clone());
        edges.entry(b).or_insert_with(Vec::new).push(a);
        edges
    })
}

fn part_one(s: &str) -> String {
    let end = Cave::Small("end".into());
    let edges = parse_input(s);
    let mut queue: VecDeque<Path> = VecDeque::new();

    {
        let mut p = Path::default();
        p.visit(&Cave::Small("start".into()))
            .expect("unable to start");
        queue.push_front(p);
    }

    let mut completed: Vec<HashSet<Cave>> = Vec::new();
    while let Some(p) = queue.pop_front() {
        if p.last() == Some(&end) {
            completed.push(p.breadcrumbs.into_iter().collect());
            continue;
        }

        let possible = if let Some(ps) = p.connections(&edges) {
            ps
        } else {
            continue;
        };

        for connection in possible {
            let mut next = p.clone();
            if next.visit(&connection).is_ok() {
                queue.push_front(next);
            }
        }
    }

    let output = completed.len();

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let _input = parse_input(s);
    let output = 0;

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day12.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_12_1_1, part_one, 12, 1, 1);
    test_example!(example_12_1_2, part_one, 12, 1, 2);
    test_example!(example_12_1_3, part_one, 12, 1, 3);
    test_example!(example_12_2_2, part_two, 12, 2, 1);
}
