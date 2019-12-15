use anyhow::{anyhow, Context, Result};
use std::collections::VecDeque;

const OP_ADDI: i64 = 1;
const OP_MULT: i64 = 2;
const OP_INPU: i64 = 3;
const OP_OUTP: i64 = 4;
const OP_JMPT: i64 = 5;
const OP_JMPF: i64 = 6;
const OP_LESS: i64 = 7;
const OP_EQUA: i64 = 8;
const OP_ADRB: i64 = 9;
const OP_TERM: i64 = 99;
const MODE_POSITION: i64 = 0;
const MODE_IMMEDIATE: i64 = 1;
const MODE_RELATIVE: i64 = 2;

#[derive(Debug)]
enum Op {
    Add(Param, Param, Param),
    Multiply(Param, Param, Param),
    JumpTrue(Param, Param),
    JumpFalse(Param, Param),
    Less(Param, Param, Param),
    Equal(Param, Param, Param),
    AdjustRelBase(Param),
    Input(Param),
    Output(Param),
    Terminate,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add(a, b, c) => write!(f, "ADD {:>10}, {:>10}, {:>10}", a, b, c),
            Op::Multiply(a, b, c) => write!(f, "MUL {:>10}, {:>10}, {:>10}", a, b, c),
            Op::JumpTrue(a, b) => write!(f, "JNZ {:>10}, {:>10}", a, b),
            Op::JumpFalse(a, b) => write!(f, "JZ  {:>10}, {:>10}", a, b),
            Op::Less(a, b, c) => write!(f, "LTH {:>10}, {:>10}, {:>10}", a, b, c),
            Op::Equal(a, b, c) => write!(f, "EQL {:>10}, {:>10}, {:>10}", a, b, c),
            Op::Input(a) => write!(f, "INP {:>10}", a),
            Op::Output(a) => write!(f, "OUT {:>10}", a),
            Op::AdjustRelBase(a) => write!(f, "ARB {}", a),
            Op::Terminate => write!(f, "END"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Suspended(i64),
    Terminated(Option<i64>),
    AwaitingInput,
}

enum InstrResult {
    Suspend(i64),
    AwaitInput,
    Terminate,
    Continue,
}

#[derive(Debug, Copy, Clone)]
enum Param {
    Immediate(i64),
    Pointer(usize),
    Relative(i64),
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Param::Immediate(x) => write!(f, "{}", x),
            Param::Pointer(x) => write!(f, "&{}", x),
            Param::Relative(x) => write!(f, "~{}", x),
        }
    }
}

#[derive(Clone)]
pub struct Interpretor {
    ip: usize,
    rb: usize,
    memory: Vec<i64>,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
}

impl Interpretor {
    pub fn new(memory: &[i64]) -> Self {
        Interpretor {
            ip: 0,
            rb: 0,
            memory: memory.to_owned(),
            outputs: Vec::new(),
            inputs: VecDeque::new(),
        }
    }

    pub fn run(&mut self) -> Result<State> {
        loop {
            let op = self.parse_op()?;
            let result = self.interpret(op)?;
            match result {
                InstrResult::Suspend(x) => return Ok(State::Suspended(x)),
                InstrResult::Terminate => {
                    return Ok(State::Terminated(self.outputs.iter().last().copied()))
                }
                InstrResult::AwaitInput => return Ok(State::AwaitingInput),
                InstrResult::Continue => continue,
            }
        }
    }

    pub fn run_complete(&mut self) -> Result<Option<i64>> {
        loop {
            match self.run()? {
                State::Terminated(x) => return Ok(x),
                State::AwaitingInput => return Err(anyhow!("expected input")),
                State::Suspended(_) => continue,
            }
        }
    }

    fn interpret(&mut self, op: Op) -> Result<InstrResult> {
        match op {
            Op::Add(a, b, c) => {
                self.set(c, self.get_value(a) + self.get_value(b))?;
                Ok(InstrResult::Continue)
            }
            Op::Multiply(a, b, c) => {
                self.set(c, self.get_value(a) * self.get_value(b))?;
                Ok(InstrResult::Continue)
            }
            Op::JumpTrue(a, b) => {
                if self.get_value(a) != 0 {
                    self.ip = self.get_value(b) as usize;
                }
                Ok(InstrResult::Continue)
            }
            Op::JumpFalse(a, b) => {
                if self.get_value(a) == 0 {
                    self.ip = self.get_value(b) as usize;
                }
                Ok(InstrResult::Continue)
            }
            Op::Less(a, b, c) => {
                self.set(
                    c,
                    if self.get_value(a) < self.get_value(b) {
                        1
                    } else {
                        0
                    },
                )?;
                Ok(InstrResult::Continue)
            }
            Op::Equal(a, b, c) => {
                self.set(
                    c,
                    if self.get_value(a) == self.get_value(b) {
                        1
                    } else {
                        0
                    },
                )?;
                Ok(InstrResult::Continue)
            }
            Op::Input(a) => {
                match self.inputs.pop_front() {
                    Some(v) => {
                        self.set(a, v)?;
                        Ok(InstrResult::Continue)
                    }
                    None => {
                        // Push Input operator back so that it gets processed again when the
                        // interpretor is resumed
                        self.ip = self.ip - 2;
                        Ok(InstrResult::AwaitInput)
                    }
                }
            }
            Op::Output(out) => {
                let v = self.get_value(out);
                self.outputs.push(v);
                Ok(InstrResult::Suspend(v))
            }
            Op::AdjustRelBase(a) => {
                self.rb = (self.rb as i64 + self.get_value(a)) as usize;
                Ok(InstrResult::Continue)
            }
            Op::Terminate => Ok(InstrResult::Terminate),
        }
    }

    fn parse_op(&mut self) -> Result<Op> {
        let mut instr = self.get_token().context("expecting instruction")?;
        let opcode = instr % 100;
        instr /= 100;

        match opcode {
            OP_ADDI => self.parse_instr_addi(&mut instr),
            OP_MULT => self.parse_instr_mult(&mut instr),
            OP_JMPT => self.parse_instr_jmpt(&mut instr),
            OP_JMPF => self.parse_instr_jmpf(&mut instr),
            OP_LESS => self.parse_instr_less(&mut instr),
            OP_EQUA => self.parse_instr_equa(&mut instr),
            OP_INPU => self.parse_instr_inpu(&mut instr),
            OP_OUTP => self.parse_instr_outp(&mut instr),
            OP_ADRB => self.parse_instr_adrb(&mut instr),
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
            Param::Pointer(i) => self.get(i),
            Param::Relative(d) => self.get((self.rb as i64 + d) as usize),
        }
    }

    fn set(&mut self, addr: Param, x: i64) -> Result<()> {
        let dst = match addr {
            Param::Pointer(x) => Ok(x),
            Param::Relative(x) => Ok((self.rb as i64 + x) as usize),
            Param::Immediate(_) => Err(anyhow!("destination can not be an immediate value")),
        }?;
        self.memory
            .resize_with(self.memory.len().max(dst + 1), || 0);
        self.memory[dst] = x;

        Ok(())
    }

    pub fn get(&self, addr: usize) -> i64 {
        if addr >= self.memory.len() {
            0
        } else {
            self.memory[addr]
        }
    }

    pub fn input(&mut self, v: i64) {
        self.inputs.push_back(v);
    }

    fn parse_instr_addi(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Add(
            self.parse_param(&mut modes)
                .context("first operand to add operation")?,
            self.parse_param(&mut modes)
                .context("second operand to add operation")?,
            self.parse_param(&mut modes)
                .context("third operand to add operation")?,
        ))
    }

    fn parse_instr_mult(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Multiply(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
        ))
    }

    fn parse_instr_jmpt(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::JumpTrue(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
        ))
    }

    fn parse_instr_jmpf(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::JumpFalse(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
        ))
    }

    fn parse_instr_less(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Less(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
        ))
    }

    fn parse_instr_equa(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Equal(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
        ))
    }

    fn parse_instr_adrb(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::AdjustRelBase(self.parse_param(&mut modes)?))
    }

    fn parse_instr_inpu(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Input(self.parse_param(&mut modes)?))
    }

    fn parse_instr_outp(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Output(self.parse_param(&mut modes)?))
    }

    fn parse_param(&mut self, modes: &mut i64) -> Result<Param> {
        let mode = *modes % 10;
        *modes /= 10;

        let p = self.get_token().context("expected param")?;

        match mode {
            MODE_IMMEDIATE => Ok(Param::Immediate(p)),
            MODE_POSITION => Ok(Param::Pointer(p as usize)),
            MODE_RELATIVE => Ok(Param::Relative(p)),
            _ => Err(anyhow!("inavlid mode: {}", mode)),
        }
    }
}
