use crate::intcode::{Interpretor, State};
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
        p.run_complete().map(|x| x.unwrap()).unwrap()
    })
}

fn get_chained_signal(phases: &[i64], memory: &[i64]) -> Result<i64> {
    let mut amplifiers: Vec<_> = phases
        .iter()
        .map(|&phase| {
            let mut i = Interpretor::new(&memory);
            i.input(phase);
            i
        })
        .collect();

    let last_amplifier = amplifiers.len() - 1;
    let mut signal = 0;
    loop {
        for (i, a) in amplifiers.iter_mut().enumerate() {
            a.input(signal);
            match a.run()? {
                State::Suspended(x) => signal = x,
                State::Terminated(x) => {
                    if i == last_amplifier {
                        return Ok(x.unwrap());
                    } else {
                        continue;
                    }
                }
                State::AwaitingInput => return Err(anyhow!("expected input")),
            }
        }
    }
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
    let mut phases: Vec<i64> = (5..=9).collect();
    let permutations = Heap::new(&mut phases);

    permutations
        .map(|ps| get_chained_signal(&ps, memory).unwrap())
        .max()
        .ok_or_else(|| anyhow!("unable to find maximum signal"))
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

    #[test]
    fn examples_2() {
        assert_eq!(
            139629729,
            answer_2(&input_generator(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            ))
            .unwrap()
        );

        assert_eq!(
            18216,
            answer_2(&input_generator(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            ))
            .unwrap()
        );
    }
}
