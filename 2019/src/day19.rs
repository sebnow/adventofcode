use crate::intcode;
use anyhow::{anyhow, Result};

type Point = aocutil::Point<i64>;

fn is_in_beam(input: &[i64], p: &Point) -> Result<bool> {
    let mut prg = intcode::Interpretor::new(input);
    prg.input(p.x);
    prg.input(p.y);

    match prg.run()? {
        intcode::State::Terminated(_) => Err(anyhow!("unexpectedly terminated")),
        intcode::State::AwaitingInput => Err(anyhow!("unexectedly awaiting input")),
        intcode::State::Suspended(x) => {
            print!("{}", x);
            Ok(x == 1)
        }
    }
}

fn find_square(input: &[i64], size: i64) -> Result<i64> {
    let mut p = Point::new(0, 0);
    loop {
        assert!(
            !is_in_beam(input, &Point::new(p.x - 1, p.y))?,
            format!("{} is in beam", p)
        );
        assert!(is_in_beam(input, &p)?, format!("{} is not in beam", p));

        let origin = Point::new(p.x, p.y - size - 1);
        let bounds = [
            origin,
            Point::new(p.x + size - 1, p.y),
            Point::new(p.x + size - 1, p.y - size - 1),
        ];

        if bounds.iter().all(|p| is_in_beam(input, p).unwrap()) {
            return Ok((origin.x * 10_000) + origin.y);
        }

        p.y += 1;
        while !is_in_beam(input, &p)? {
            p.x += 1;
        }
    }
}

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
    for y in 0..50 {
        for x in 0..50 {
            if is_in_beam(input, &Point::new(x, y))? {
                sum += 1;
            }
        }
        println!();
    }
    Ok(sum)
}

//fn gradient(input: &[i64]) -> Result<f64> {
//    let mut p = Point::new(0, 25);
//    for x in 0..100 {
//        p.x = x;
//        if is_in_beam(input, &p)? {
//            return Ok(p.y as f64 / p.x as f64);
//        }
//    }
//
//    Err(anyhow!("beam not found"))
//}

#[aoc(day19, part2)]
fn answer_2(input: &[i64]) -> Result<i64> {
    find_square(input, 100)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer_2() {
        let input = input_generator(&std::fs::read_to_string("input/2019/day19.txt").unwrap());
        assert_eq!(260_049, find_square(&input, 5).unwrap());
    }
}
