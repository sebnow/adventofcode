use anyhow::{anyhow, Context, Result};

const OP_ADDI: i64 = 1;
const OP_MULT: i64 = 2;
const OP_INPU: i64 = 3;
const OP_OUTP: i64 = 4;
const OP_TERM: i64 = 99;
const MODE_POSITION: i64 = 0;
const MODE_IMMEDIATE: i64 = 1;

#[derive(Debug)]
enum Op {
    Add(Param, Param, usize),
    Multiply(Param, Param, usize),
    Input(usize),
    Output(Param),
    Terminate,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add(a, b, c) => write!(f, "&{} = {} + {}", c, a, b),
            Op::Multiply(a, b, c) => write!(f, "&{} = {} * {}", c, a, b),
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
            print!("\t\t\t");

            if let Op::Terminate = op {
                break;
            }

            self.interpret(op)?;
        }

        Ok(self.outputs.to_owned())
    }

    pub fn interpret(&mut self, op: Op) -> Result<()> {
        print!("[{:064}] ", op);
        match op {
            Op::Add(a, b, out) => {
                self.memory[out] = self.get_value(a) + self.get_value(b);
                Ok(())
            }
            Op::Multiply(a, b, out) => {
                self.memory[out] = self.get_value(a) * self.get_value(b);
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
                println!();
                println!("==========================================");
                println!("Output: {} ", v);
                println!("==========================================");
                Ok(())
            }
            _ => Err(anyhow!("unexpected terminate operation")),
        }?;

        Ok(())
    }

    fn parse_op(&mut self) -> Result<Op> {
        println!();
        let mut instr = self.get_token().context("expecting instruction")?;
        let opcode = instr % 100;
        instr /= 100;

        match opcode {
            OP_ADDI => self.parse_instr_add(&mut instr),
            OP_MULT => self.parse_instr_mul(&mut instr),
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
            print!("{:6} ", v);
            self.ip += 1;
            Ok(v)
        }
    }

    fn get_value(&self, param: Param) -> i64 {
        match param {
            Param::Immediate(v) => v,
            Param::Pointer(i) => {
                print!("(&{}={})", i, self.memory[i]);
                self.memory[i]
            }
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
fn answer_2(memory: &[i64]) -> i64 {
    0
}
