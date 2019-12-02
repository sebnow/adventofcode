#[derive(Debug)]
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
        use OpCode::*;

        match self {
            Add(a, b) => a + b,
            Multiply(a, b) => a * b,
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Add(a, b) => write!(f, "{} + {}", a, b),
            OpCode::Multiply(a, b) => write!(f, "{} * {}", a, b),
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

fn execute(input: &[i64]) -> Vec<i64> {
    let mut registers = input.to_owned();
    let mut pc = 0;

    loop {
        if registers[pc] == 99 {
            break;
        }

        run(pc, &mut registers);
        pc += 4;
    }

    registers
}

fn execute_with_inputs(input: &[i64], a: i64, b: i64) -> Vec<i64> {
    let mut registers = input.to_owned();
    registers[1] = a;
    registers[2] = b;

    execute(&registers)
}

fn run(pc: usize, registers: &mut Vec<i64>) {
    let code = registers[pc];

    if code == 99 {
        return;
    }

    let a = registers[pc + 1];
    let b = registers[pc + 2];
    let op = OpCode::new(code, registers[a as usize], registers[b as usize]);
    let out = registers[pc + 3] as usize;

    if out > registers.len() {
        registers.resize(out + 1, 0);
    }
    registers[out] = op.execute();
}

#[aoc(day2, part1)]
fn answer_1(input: &[i64]) -> i64 {
    let program = execute_with_inputs(&input, 12, 2);

    program[0]
}

#[aoc(day2, part2)]
fn answer_2(input: &[i64]) -> i64 {
    for x in 0..99 {
        for y in 0..99 {
            let program = execute_with_inputs(&input, x, y);
            if program[0] == 19690720 {
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
        assert_eq!(vec![2, 0, 0, 0, 99], execute(&vec![1, 0, 0, 0, 99]));
        assert_eq!(vec![2, 3, 0, 6, 99], execute(&vec![2, 3, 0, 3, 99]));
        assert_eq!(
            vec![2, 4, 4, 5, 99, 9801],
            execute(&vec![2, 4, 4, 5, 99, 9801])
        );
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            execute(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }
}
