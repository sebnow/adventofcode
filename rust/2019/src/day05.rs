use crate::intcode::Interpretor;
use anyhow::Result;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()))
        .flatten()
        .collect()
}

#[aoc(day5, part1)]
fn answer_1(memory: &[i64]) -> Result<i64> {
    let mut proc = Interpretor::new(&memory);
    proc.input(1);
    proc.run_complete().map(|x| x.unwrap())
}

#[aoc(day5, part2)]
fn answer_2(memory: &[i64]) -> Result<i64> {
    let mut proc = Interpretor::new(&memory);
    proc.input(5);
    proc.run_complete().map(|x| x.unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::intcode::State;

    #[test]
    fn examples_2_1() {
        let input = input_generator("3,9,8,9,10,9,4,9,99,-1,8");

        let mut proc = Interpretor::new(&input);
        proc.input(8);
        assert_eq!(State::Suspended(1), proc.run().unwrap());

        let mut proc = Interpretor::new(&input);
        proc.input(10);
        assert_eq!(State::Suspended(0), proc.run().unwrap());
    }

    #[test]
    fn examples_2_2() {
        let input = input_generator("3,9,7,9,10,9,4,9,99,-1,8");
        let mut proc = Interpretor::new(&input);
        proc.input(7);
        assert_eq!(State::Suspended(1), proc.run().unwrap());

        let mut proc = Interpretor::new(&input);
        proc.input(8);
        assert_eq!(State::Suspended(0), proc.run().unwrap());
    }

    #[test]
    fn examples_2_3() {
        let input = input_generator("3,3,1108,-1,8,3,4,3,99");
        let mut proc = Interpretor::new(&input);
        proc.input(8);
        assert_eq!(State::Suspended(1), proc.run().unwrap());

        let mut proc = Interpretor::new(&input);
        proc.input(9);
        assert_eq!(State::Suspended(0), proc.run().unwrap());
    }

    #[test]
    fn examples_2_4() {
        let input = input_generator("3,3,1107,-1,8,3,4,3,99");
        let mut proc = Interpretor::new(&input);
        proc.input(7);
        assert_eq!(State::Suspended(1), proc.run().unwrap());

        let mut proc = Interpretor::new(&input);
        proc.input(8);
        assert_eq!(State::Suspended(0), proc.run().unwrap());
    }

    #[test]
    fn examples_2_5() {
        let input = input_generator("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut proc = Interpretor::new(&input);
        proc.input(0);
        assert_eq!(State::Suspended(0), proc.run().unwrap());

        let mut proc = Interpretor::new(&input);
        proc.input(1);
        assert_eq!(State::Suspended(1), proc.run().unwrap());
    }

    #[test]
    fn examples_2_6() {
        let input = input_generator("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let mut proc = Interpretor::new(&input);
        proc.input(7);
        assert_eq!(State::Suspended(999), proc.run().unwrap());

        let mut proc = Interpretor::new(&input);
        proc.input(8);
        assert_eq!(State::Suspended(1000), proc.run().unwrap());

        let mut proc = Interpretor::new(&input);
        proc.input(9);
        assert_eq!(State::Suspended(1001), proc.run().unwrap());
    }
}
