use crate::intcode::Interpretor;
use anyhow::Result;
use itertools::Itertools;

fn run_with_input(memory: &[i64], a: i64, b: i64) -> Result<i64> {
    let mut mem = memory.to_owned();
    mem[1] = a;
    mem[2] = b;

    let mut prg = Interpretor::new(&mem);
    prg.run_complete()?;
    Ok(prg.get(0))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day2, part1)]
fn answer_1(memory: &[i64]) -> Result<i64> {
    run_with_input(memory, 12, 2)
}

#[aoc(day2, part2)]
fn answer_2(memory: &[i64]) -> i64 {
    (0..99)
        .cartesian_product(0..99)
        .filter_map(|(x, y)| {
            let result = run_with_input(memory, x, y).unwrap();
            if result == 19_690_720 {
                Some(100 * x + y)
            } else {
                None
            }
        })
        .last()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(memory: &[i64]) -> i64 {
        let mut prg = Interpretor::new(memory);
        prg.run_complete().unwrap().unwrap()
    }

    #[test]
    fn examples_1() {
        assert_eq!(2, run(&[1, 0, 0, 0, 4, 0, 99]));
        assert_eq!(6, run(&[2, 3, 0, 3, 4, 3, 99]));
        assert_eq!(9801, run(&[2, 6, 6, 7, 4, 7, 99, 0]));
        assert_eq!(30, run(&[1, 1, 1, 4, 99, 5, 6, 0, 4, 0, 99]));
    }
}
