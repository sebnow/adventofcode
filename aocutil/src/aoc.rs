use aoc_helper::{AocDay, Puzzle};
use std::collections::HashMap;

type Solver = fn(String) -> String;

pub struct AdventOfCode {
    solvers: HashMap<(u32, u32), (Solver, Solver)>,
}

impl AdventOfCode {
    pub fn new() -> Self {
        AdventOfCode {
            solvers: HashMap::default(),
        }
    }

    pub fn register(&mut self, year: u32, day: u32, part1: Solver, part2: Solver) {
        self.solvers.insert((year, day), (part1, part2));
    }

    pub fn run_day(&self, year: u32, day: u32) {
        let solvers = self.solvers.get(&(year, day)).unwrap();

        let mut helper = AocDay::new(year as i32, day as u8);
        let part1 = Puzzle::new(1, solvers.0);
        let part2 = Puzzle::new(2, solvers.1);

        helper.run(&part1).unwrap();
        helper.run(&part2).unwrap();
    }
}

#[macro_export]
macro_rules! aoc {
    ($year:expr, $day:expr, $part1:expr, $part2:expr) => {
        pub fn register(aoc: &mut aocutil::AdventOfCode) {
            aoc.register($year, $day, $part1, $part2);
        }
    }
}
