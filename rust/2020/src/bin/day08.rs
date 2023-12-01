use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Instr {
    Jmp(i64),
    Acc(i64),
    Nop(i64),
}

fn parse_input(s: &str) -> Result<Vec<Instr>> {
    s.lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let instr = parts.next().ok_or_else(|| anyhow!("instruction missing"))?;
            let arg = parts
                .next()
                .ok_or_else(|| anyhow!("argument missing"))?
                .parse()?;
            match instr {
                "acc" => Ok(Instr::Acc(arg)),
                "jmp" => Ok(Instr::Jmp(arg)),
                "nop" => Ok(Instr::Nop(arg)),
                instr => Err(anyhow!("invalid instruction {}", instr)),
            }
        })
        .collect()
}

fn part_one(input: &str) -> String {
    let program = parse_input(input).unwrap();
    let mut history: HashSet<usize> = HashSet::default();
    let mut pc: usize = 0;
    let mut acc: i64 = 0;

    while history.insert(pc) {
        let mut jmp: i64 = 1;

        match program[pc] {
            Instr::Acc(v) => acc += v,
            Instr::Jmp(v) => jmp = v,
            _ => {}
        }

        pc = (pc as i64 + jmp) as usize;
    }

    acc.to_string()
}

fn part_two(input: &str) -> String {
    let original = parse_input(input).unwrap();

    original
        .iter()
        .enumerate()
        .fold(Vec::default(), |mut programs, (pc, &instr)| {
            let new_instr = match instr {
                Instr::Nop(v) => Instr::Jmp(v),
                Instr::Jmp(v) => Instr::Nop(v),
                _ => return programs,
            };

            let mut new_program = original.clone();
            new_program[pc] = new_instr;

            programs.push(new_program);
            programs
        })
        .iter()
        .find_map(|program| {
            let mut history: HashSet<usize> = HashSet::default();
            let mut pc: usize = 0;
            let mut acc: i64 = 0;

            while pc < program.len() {
                if !history.insert(pc) {
                    return None;
                }

                let mut jmp: i64 = 1;

                match program[pc] {
                    Instr::Acc(v) => acc += v,
                    Instr::Jmp(v) => jmp = v,
                    _ => {}
                }

                pc = (pc as i64 + jmp) as usize;
            }

            Some(acc)
        })
        .unwrap()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day08.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 8, 1, 1);
    test_example!(example_two_1, part_two, 8, 2, 1);
}
