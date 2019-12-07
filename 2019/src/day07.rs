use crate::intcode::Interpretor;
use anyhow::{anyhow, Result};
use permutohedron::Heap;

fn get_signal(phases: &[i64], memory: &[i64]) -> i64 {
    let amplifiers = phases.iter().map(|&phase| {
        let mut p = Interpretor::new(&memory);
        p.input(phase);
        p
    });

    amplifiers.fold(0, |signal, mut p| {
        p.input(signal);
        let outputs = p.run().unwrap();
        *outputs.iter().last().unwrap()
    })
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day7, part1)]
fn answer_1(memory: &[i64]) -> Result<i64> {
    let mut phases: Vec<i64> = (0..=4).collect();
    let permutations = Heap::new(&mut phases);

    permutations
        .map(|ps| get_signal(&ps, memory))
        .max()
        .ok_or_else(|| anyhow!("unable to find maximum signal"))
}

#[aoc(day7, part2)]
fn answer_2(memory: &[i64]) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(
            43210,
            answer_1(&input_generator(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            ))
            .unwrap()
        );

        assert_eq!(
            54321,
            answer_1(&input_generator(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            ))
            .unwrap()
        );

        assert_eq!(
            65210,
            answer_1(&input_generator(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )).unwrap()
        );
    }
}
