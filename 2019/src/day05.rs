use anyhow::{anyhow, Context, Result};

const OP_ADDI: i64 = 1;
const OP_MULT: i64 = 2;
const OP_INPU: i64 = 3;
const OP_OUTP: i64 = 4;
const OP_JMPT: i64 = 5;
const OP_JMPF: i64 = 6;
const OP_LESS: i64 = 7;
const OP_EQUA: i64 = 8;
const OP_TERM: i64 = 99;
const MODE_POSITION: i64 = 0;
const MODE_IMMEDIATE: i64 = 1;

#[derive(Debug)]
enum Op {
    Add(Param, Param, usize),
    Multiply(Param, Param, usize),
    JumpTrue(Param, Param),
    JumpFalse(Param, Param),
    Less(Param, Param, usize),
    Equal(Param, Param, usize),
    Input(usize),
    Output(Param),
    Terminate,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add(a, b, c) => write!(f, "&{} = {} + {}", c, a, b),
            Op::Multiply(a, b, c) => write!(f, "&{} = {} * {}", c, a, b),
            Op::JumpTrue(a, b) => write!(f, "JT {} {}", a, b),
            Op::JumpFalse(a, b) => write!(f, "JF {} {}", a, b),
            Op::Less(a, b, c) => write!(f, "{} = {} < {}", c, a, b),
            Op::Equal(a, b, c) => write!(f, "{} = {} == {}", c, a, b),
            Op::Input(a) => write!(f, "->{}", a),
            Op::Output(a) => write!(f, "<-{}", a),
            Op::Terminate => write!(f, "TERM"),
        }
    }
}

#[derive(Debug)]
enum Param {
    Immediate(i64),
    Pointer(usize),
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Param::Immediate(x) => write!(f, "{}", x),
            Param::Pointer(x) => write!(f, "&{}", x),
        }
    }
}

struct Processor {
    ip: usize,
    memory: Vec<i64>,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl Processor {
    pub fn new(memory: &[i64]) -> Self {
        Processor {
            ip: 0,
            memory: memory.to_owned(),
            outputs: Vec::new(),
            inputs: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Vec<i64>> {
        loop {
            let op = self.parse_op()?;

            if let Op::Terminate = op {
                break;
            }

            self.interpret(op)?;
        }

        Ok(self.outputs.to_owned())
    }

    pub fn interpret(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add(a, b, out) => {
                self.memory[out] = self.get_value(a) + self.get_value(b);
                Ok(())
            }
            Op::Multiply(a, b, out) => {
                self.memory[out] = self.get_value(a) * self.get_value(b);
                Ok(())
            }
            Op::JumpTrue(a, b) => {
                if self.get_value(a) != 0 {
                    self.ip = self.get_value(b) as usize;
                }
                Ok(())
            }
            Op::JumpFalse(a, b) => {
                if self.get_value(a) == 0 {
                    self.ip = self.get_value(b) as usize;
                }
                Ok(())
            }
            Op::Less(a, b, out) => {
                self.memory[out] = if self.get_value(a) < self.get_value(b) {
                    1
                } else {
                    0
                };
                Ok(())
            }
            Op::Equal(a, b, out) => {
                self.memory[out] = if self.get_value(a) == self.get_value(b) {
                    1
                } else {
                    0
                };
                Ok(())
            }
            Op::Input(out) => {
                let v = self.inputs.pop().ok_or_else(|| anyhow!("missing input"))?;
                self.memory[out] = v;
                Ok(())
            }
            Op::Output(out) => {
                let v = self.get_value(out);
                self.outputs.push(v);
                Ok(())
            }
            _ => Err(anyhow!("unexpected terminate operation")),
        }?;

        Ok(())
    }

    fn parse_op(&mut self) -> Result<Op> {
        let mut instr = self.get_token().context("expecting instruction")?;
        let opcode = instr % 100;
        instr /= 100;

        match opcode {
            OP_ADDI => self.parse_instr_add(&mut instr),
            OP_MULT => self.parse_instr_mul(&mut instr),
            OP_JMPT => self.parse_instr_jt(&mut instr),
            OP_JMPF => self.parse_instr_jf(&mut instr),
            OP_LESS => self.parse_instr_less(&mut instr),
            OP_EQUA => self.parse_instr_equal(&mut instr),
            OP_INPU => self.parse_instr_input(),
            OP_OUTP => self.parse_instr_output(&mut instr),
            OP_TERM => Ok(Op::Terminate),
            _ => Err(anyhow!("invalid operation: {}", opcode)),
        }
        .with_context(|| format!("opcode={},modes={}", opcode, instr))
    }

    fn get_token(&mut self) -> Result<i64> {
        if self.ip >= self.memory.len() {
            Err(anyhow!("expected token"))
        } else {
            let v = self.memory[self.ip];
            self.ip += 1;
            Ok(v)
        }
    }

    fn get_value(&self, param: Param) -> i64 {
        match param {
            Param::Immediate(v) => v,
            Param::Pointer(i) => self.memory[i],
        }
    }

    fn input(&mut self, v: i64) {
        self.inputs.push(v);
    }

    fn parse_instr_add(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Add(
            self.parse_param(&mut modes)
                .context("first operand to add operation")?,
            self.parse_param(&mut modes)
                .context("second operand to add operation")?,
            self.parse_position(&mut modes)
                .context("third operand to add operation")?,
        ))
    }

    fn parse_instr_mul(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Multiply(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
            self.parse_position(&mut modes)?,
        ))
    }

    fn parse_instr_jt(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::JumpTrue(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
        ))
    }

    fn parse_instr_jf(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::JumpFalse(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
        ))
    }

    fn parse_instr_less(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Less(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
            self.parse_position(&mut modes)?,
        ))
    }

    fn parse_instr_equal(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Equal(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
            self.parse_position(&mut modes)?,
        ))
    }

    fn parse_instr_input(&mut self) -> Result<Op> {
        Ok(Op::Input(self.get_token()? as usize))
    }

    fn parse_instr_output(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Output(self.parse_param(&mut modes)?))
    }

    fn parse_param(&mut self, modes: &mut i64) -> Result<Param> {
        let mode = *modes % 10;
        *modes /= 10;

        let p = self.get_token().context("expected param")?;

        match mode {
            MODE_IMMEDIATE => Ok(Param::Immediate(p)),
            MODE_POSITION => Ok(Param::Pointer(p as usize)),
            _ => Err(anyhow!("inavlid mode: {}", mode)),
        }
    }

    fn parse_position(&mut self, modes: &mut i64) -> Result<usize> {
        let mode = *modes % 10;
        *modes /= 10;

        match mode {
            MODE_POSITION => {
                let p = self.get_token()?;
                Ok(p as usize)
            }
            _ => Err(anyhow!("output parameter must have position mode")),
        }
    }
}

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
    let mut proc = Processor::new(&memory);
    proc.input(1);
    let outputs = proc.run()?;
    Ok(*outputs
        .iter()
        .last()
        .ok_or_else(|| anyhow!("expected output"))?)
}

#[aoc(day5, part2)]
fn answer_2(memory: &[i64]) -> Result<i64> {
    let mut proc = Processor::new(&memory);
    proc.input(5);
    let outputs = proc.run()?;
    Ok(*outputs
        .iter()
        .last()
        .ok_or_else(|| anyhow!("expected output"))?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_2_1() {
        let input = input_generator("3,9,8,9,10,9,4,9,99,-1,8");

        let mut proc = Processor::new(&input);
        proc.input(8);
        assert_eq!(vec![1], proc.run().unwrap());

        let mut proc = Processor::new(&input);
        proc.input(10);
        assert_eq!(vec![0], proc.run().unwrap());
    }

    #[test]
    fn examples_2_2() {
        let input = input_generator("3,9,7,9,10,9,4,9,99,-1,8");
        let mut proc = Processor::new(&input);
        proc.input(7);
        assert_eq!(vec![1], proc.run().unwrap());

        let mut proc = Processor::new(&input);
        proc.input(8);
        assert_eq!(vec![0], proc.run().unwrap());
    }

    #[test]
    fn examples_2_3() {
        let input = input_generator("3,3,1108,-1,8,3,4,3,99");
        let mut proc = Processor::new(&input);
        proc.input(8);
        assert_eq!(vec![1], proc.run().unwrap());

        let mut proc = Processor::new(&input);
        proc.input(9);
        assert_eq!(vec![0], proc.run().unwrap());
    }

    #[test]
    fn examples_2_4() {
        let input = input_generator("3,3,1107,-1,8,3,4,3,99");
        let mut proc = Processor::new(&input);
        proc.input(7);
        assert_eq!(vec![1], proc.run().unwrap());

        let mut proc = Processor::new(&input);
        proc.input(8);
        assert_eq!(vec![0], proc.run().unwrap());
    }

    #[test]
    fn examples_2_5() {
        let input = input_generator("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let mut proc = Processor::new(&input);
        proc.input(0);
        assert_eq!(vec![0], proc.run().unwrap());

        let mut proc = Processor::new(&input);
        proc.input(1);
        assert_eq!(vec![1], proc.run().unwrap());
    }

    #[test]
    fn examples_2_6() {
        let input = input_generator("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let mut proc = Processor::new(&input);
        proc.input(7);
        assert_eq!(vec![999], proc.run().unwrap());

        let mut proc = Processor::new(&input);
        proc.input(8);
        assert_eq!(vec![1000], proc.run().unwrap());

        let mut proc = Processor::new(&input);
        proc.input(9);
        assert_eq!(vec![1001], proc.run().unwrap());
    }
}
