use anyhow::anyhow;
use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr};

type Registers = [i64; 4];

#[derive(Debug, Clone, Copy)]
enum Param {
    Var(char),
    Lit(i64),
}

impl FromStr for Param {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s.chars().nth(0).ok_or_else(|| anyhow!("empty string"))? {
                c @ 'w'..='z' => Param::Var(c),
                _ => Param::Lit(s.parse()?),
            },
        )
    }
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Inp(char),
    Add(char, Param),
    Mul(char, Param),
    Div(char, Param),
    Mod(char, Param),
    Eql(char, Param),
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let kind = parts.next().unwrap();
        let var = parts.next().unwrap().chars().nth(0).unwrap();

        match (kind, parts.next()) {
            ("inp", None) => Ok(Instr::Inp(var)),
            ("add", Some(p)) => Ok(Instr::Add(var, p.parse()?)),
            ("mul", Some(p)) => Ok(Instr::Mul(var, p.parse()?)),
            ("div", Some(p)) => Ok(Instr::Div(var, p.parse()?)),
            ("mod", Some(p)) => Ok(Instr::Mod(var, p.parse()?)),
            ("eql", Some(p)) => Ok(Instr::Eql(var, p.parse()?)),
            _ => Err(anyhow!("invalid instruction \"{}\"", s)),
        }
    }
}

#[derive(Default)]
struct Alu {
    memory: Registers,
    input: VecDeque<u8>,
}

impl Alu {
    pub fn new() -> Self {
        Alu {
            memory: [0; 4],
            ..Default::default()
        }
    }

    pub fn input(&mut self, input: &[u8]) {
        self.input.clear();
        self.input.extend(input.iter());
    }

    pub fn execute(&mut self, instr: &Instr) {
        use Instr::*;

        match *instr {
            Inp(reg) => {
                let p = self.get_input();
                self.set(reg, p);
            }
            Add(reg, p) => self.set(reg, Param::Lit(self.get(reg) + self.get_param(p))),
            Mul(reg, p) => self.set(reg, Param::Lit(self.get(reg) * self.get_param(p))),
            Div(reg, p) => self.set(reg, Param::Lit(self.get(reg) - self.get_param(p))),
            Mod(reg, p) => self.set(reg, Param::Lit(self.get(reg) % self.get_param(p))),
            Eql(reg, p) => self.set(reg, Param::Lit((self.get(reg) == self.get_param(p)) as i64)),
        }
    }

    pub fn run<I: Iterator<Item = Instr>>(&mut self, instr: I) {
        for i in instr {
            self.execute(&i);
        }
    }

    #[inline]
    fn get_input(&mut self) -> Param {
        Param::Lit(self.input.pop_front().expect("requires input") as i64)
    }

    fn set(&mut self, reg: char, param: Param) {
        let idx = reg as usize - 'w' as usize;

        match param {
            Param::Lit(l) => self.memory[idx] = l,
            Param::Var(v) => self.memory[idx] = self.get(v),
        }
    }

    pub fn get_param(&self, param: Param) -> i64 {
        match param {
            Param::Lit(l) => l,
            Param::Var(v) => self.get(v),
        }
    }

    pub fn get(&self, reg: char) -> i64 {
        self.memory[reg as usize - 'w' as usize]
    }
}

fn is_valid<I: Iterator<Item = Instr>>(program: I, model: i64) -> bool {
    let mut alu = Alu::new();
    let n = model.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect_vec();

    if n.contains(&0) {
        return false;
    }

    alu.input(&n);
    alu.run(program);
    alu.get('z') == 0
}

fn find_largest(program: Vec<Instr>) -> i64{
    let mut min = 0;
    let mut max = i64::MAX;

    loop {
        let mid = (max - min) / 2;
        if is_valid(program.iter().copied(), mid) {
            return mid
        }
    }
}

fn parse_input(s: &str) -> impl Iterator<Item = Instr> + '_ {
    s.lines().map(|l| l.parse().unwrap())
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).collect_vec();
    let output = find_largest(input);

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);

    let output = 0;

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day24.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day24 {
    use super::*;
    use aocutil::test_example;

    //test_example!(example_24_1_1, part_one, 24, 1, 1);
    //test_example!(example_24_2_1, part_two, 24, 2, 1);

    #[test]
    fn example_programs() {
        assert_eq!(
            {
                let mut alu = Alu::new();
                alu.input(&[10]);
                alu.run(parse_input("inp x\nmul x -1"));
                alu.get('x')
            },
            -10
        );

        assert_eq!(
            {
                let mut alu = Alu::new();
                alu.input(&[3, 9]);
                alu.run(parse_input("inp z\ninp x\nmul z 3\neql z x"));
                alu.get('z')
            },
            1
        );
    }
}
