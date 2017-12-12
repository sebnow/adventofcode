extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::Regex;

type PID = usize;

#[derive(Debug, PartialEq, Eq)]
struct Graph(Vec<HashSet<PID>>);

impl Graph {
    pub fn new() -> Self {
        Graph(Vec::new())
    }

    pub fn add_edge(&mut self, a: PID, b: PID) {
        if self.0.get(a).is_some() {
            self.0.get_mut(a).unwrap().insert(b);
        } else {
            let mut pids = HashSet::new();
            pids.insert(b);
            self.0.insert(a, pids);
        }
    }

    pub fn count_connected_to(&self, root: PID) -> i32 {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(root);
        seen.insert(root);

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();

            for &n in self.0.get(p).unwrap() {
                if !seen.contains(&n) {
                    seen.insert(n);
                    queue.push_back(n);
                }
            }
        }

        seen.len() as i32
    }

    pub fn groups(&self) -> i32 {
        0
    }
}

fn parse_pids(input: &str) -> Vec<usize> {
    input.split(", ").map(|x| x.parse().unwrap()).collect()
}

fn parse_input<R: BufRead>(reader: R) -> Graph {
    let mut g = Graph::new();
    let re = Regex::new(r"^(\d+) <-> ((?:(?:\d+)(?:, )?)+)$").unwrap();

    for l in reader.lines().map(|l| l.unwrap()) {
        let caps = re.captures(&l).unwrap();
        let pid = caps.get(1).unwrap().as_str().parse::<PID>().unwrap();

        for p in caps.get(2).map(|m| parse_pids(m.as_str())).unwrap() {
            g.add_edge(pid, p);
        }
    }

    g
}

fn answer_1(input: &Graph) -> i32 {
    input.count_connected_to(0)
}

fn answer_2(input: &Graph) -> i32 {
    input.groups()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let input = parse_input(reader);

    println!("Part 1: {:?}", answer_1(&input));
    println!("Part 2: {:?}", answer_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer_1() {
        let input = Cursor::new(
            "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5",
        );
        assert_eq!(answer_1(&parse_input(input)), 6);
    }

    #[test]
    fn example_answer_2() {
        let input = Cursor::new(
            "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5",
        );
        assert_eq!(answer_2(&parse_input(input)), 2);
    }
}
