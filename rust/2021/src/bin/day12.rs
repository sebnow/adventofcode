use anyhow::{anyhow, Result};
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

trait Visitor: Clone + Default {
    fn visit(&mut self, cave: &Cave);
    fn can_visit(&self, cave: &Cave) -> bool;
}

#[derive(Default, Clone)]
struct Part1Visitor {
    visited: HashSet<Cave>,
}

impl Visitor for Part1Visitor {
    fn visit(&mut self, cave: &Cave) {
        if matches!(cave, Cave::Small(_)) {
            self.visited.insert(cave.clone());
        }
    }

    fn can_visit(&self, cave: &Cave) -> bool {
        !self.visited.contains(cave)
    }
}

#[derive(Default, Clone)]
struct Part2Visitor {
    visited: HashMap<Cave, usize>,
}

impl Visitor for Part2Visitor {
    fn visit(&mut self, cave: &Cave) {
        if matches!(cave, Cave::Small(_)) {
            *self.visited.entry(cave.clone()).or_insert(0) += 1;
        }
    }

    fn can_visit(&self, cave: &Cave) -> bool {
        match cave {
            Cave::Small(c) if matches!(c.as_str(), "start" | "end") => {
                !self.visited.contains_key(cave)
            }
            Cave::Small(_) => self
                .visited
                .get(cave)
                .map(|_| self.visited.values().all(|&v| v < 2))
                .unwrap_or(true),
            Cave::Big(_) => true,
        }
    }
}

#[derive(Default, Clone)]
struct Path<V: Visitor> {
    visitor: V,
    breadcrumbs: Vec<Cave>,
}

impl<V: Visitor> Path<V> {
    pub fn new(visitor: V) -> Self {
        Path{
            visitor,
            ..Default::default()
        }
    }

    fn visit(&mut self, cave: &Cave) -> Result<()> {
        if !self.visitor.can_visit(cave) {
            return Err(anyhow!("can't visit"));
        }

        self.visitor.visit(cave);
        self.breadcrumbs.push(cave.clone());
        Ok(())
    }

    fn connections(&self, connections: &Edges) -> Option<Vec<Cave>> {
        let caves = connections.get(self.last()?)?;
        Some(
            caves
                .iter()
                .filter(|c| self.visitor.can_visit(c))
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
        edges
            .entry(a.clone())
            .or_insert_with(Vec::new)
            .push(b.clone());
        edges.entry(b).or_insert_with(Vec::new).push(a);
        edges
    })
}

fn solve<V: Visitor>(s: &str) -> usize {
    let start = Cave::Small("start".into());
    let end = Cave::Small("end".into());
    let edges = parse_input(s);
    let mut queue: VecDeque<Path<V>> = VecDeque::new();

    {
        let mut p = Path::new(V::default());
        p.visit(&start)
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

    completed.len()
}

fn part_one(s: &str) -> String {
    format!("{}", solve::<Part1Visitor>(s))
}

fn part_two(s: &str) -> String {
    format!("{}", solve::<Part2Visitor>(s))
}

fn main() {
    let input = include_str!("../../../../input/2021/day12.txt");
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
    test_example!(example_12_2_1, part_two, 12, 2, 1);
    test_example!(example_12_2_2, part_two, 12, 2, 2);
    test_example!(example_12_2_3, part_two, 12, 2, 3);
}
