const OP_TERM: i64 = 99;

fn run_with_input(memory: &[i64], a: i64, b: i64) -> Vec<i64> {
    let mut memory = memory.to_owned();
    memory[1] = a;
    memory[2] = b;

    run(&mut memory).to_owned()
}

fn run(memory: &mut [i64]) -> &[i64] {
    let mut ic = 0;

    loop {
        if memory[ic] == OP_TERM {
            break;
        }

        let op = OpCode::new(
            memory[ic],
            memory[memory[ic + 1] as usize],
            memory[memory[ic + 2] as usize],
        );
        let out = memory[ic + 3] as usize;

        memory[out] = op.execute();
        ic += 4;
    }

    memory
}

enum OpCode {
    Add(i64, i64),
    Multiply(i64, i64),
}

impl OpCode {
    fn new(code: i64, a: i64, b: i64) -> Self {
        use OpCode::*;

        match code {
            1 => Add(a, b),
            2 => Multiply(a, b),
            _ => panic!("unknown opcode: {}", code),
        }
    }

    fn execute(&self) -> i64 {
        match self {
            OpCode::Add(a, b) => a + b,
            OpCode::Multiply(a, b) => a * b,
        }
    }
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
fn answer_1(memory: &[i64]) -> i64 {
    run_with_input(memory, 12, 2)[0]
}

#[aoc(day2, part2)]
fn answer_2(memory: &[i64]) -> i64 {
    for x in 0..99 {
        for y in 0..99 {
            let result = run_with_input(memory, x, y)[0];
            if result == 19_690_720 {
                return 100 * x + y;
            }
        }
    }

    panic!("No answer");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(vec![2, 0, 0, 0, 99], run(&mut [1, 0, 0, 0, 99]));
        assert_eq!(vec![2, 3, 0, 6, 99], run(&mut [2, 3, 0, 3, 99]));
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], run(&mut [2, 4, 4, 5, 99, 9801]));
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            run(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }
}
