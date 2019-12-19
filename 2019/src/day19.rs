use crate::intcode;
use anyhow::Result;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day19, part1)]
fn answer_1(input: &[i64]) -> Result<i64> {

    let mut sum = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut prg = intcode::Interpretor::new(input);
            prg.input(x);
            prg.input(y);

            let x = match prg.run()? {
                intcode::State::Terminated(_) => break,
                intcode::State::AwaitingInput => panic!("You what? I already gave input"),
                intcode::State::Suspended(x) => x,
            };

            print!("{}", x);
            sum += x;
        }
        println!();
    }
    Ok(sum)
}

#[aoc(day19, part2)]
fn answer_2(input: &[i64]) -> Result<usize> {
    Ok(0)
}
