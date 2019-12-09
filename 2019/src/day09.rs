use crate::intcode::Interpretor;
use anyhow::Result;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day9, part1)]
fn answer_1(memory: &[i64]) -> Result<i64> {
    let mut i = Interpretor::new(memory);
    i.input(1);
    i.run_complete()
}

#[aoc(day9, part2)]
fn answer_2(memory: &[i64]) -> Result<i64> {
    let mut i = Interpretor::new(memory);
    i.input(2);
    i.run_complete()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(
            1219070632396864,
            answer_1(&input_generator("1102,34915192,34915192,7,4,7,99,0")).unwrap()
        );

        assert_eq!(
            1125899906842624,
            answer_1(&input_generator("104,1125899906842624,99")).unwrap()
        );
    }

    #[test]
    fn examples_2() {}
}
