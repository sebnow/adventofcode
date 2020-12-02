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

#[derive(Clone, Default)]
pub struct Interpretor {
    ip: usize,
    rb: usize,
    memory: Vec<i64>,
    inputs: VecDeque<i64>,
    output: Option<i64>,
}

impl Interpretor {
    pub fn new(memory: &[i64]) -> Self {
        Interpretor {
            memory: memory.to_owned(),
            ..Self::default()
        }
    }

    #[inline]
    fn param(&self, modes: i64, arg: usize) -> i64 {
        let value = self.fetch(self.ip + 1 + arg);
        let mode = (modes / 10_i64.pow(arg as u32)) % 10;
        match mode {
            MODE_IMMEDIATE => value,
            MODE_POSITION => self.fetch(value as usize),
            MODE_RELATIVE => self.fetch((value + self.rb as i64) as usize),
            _ => unreachable!("invalid mode"),
        }
    }

    #[inline]
    fn addr(&self, modes: i64, arg: usize) -> i64 {
        let value = self.fetch(self.ip + 1 + arg);
        let mode = (modes / 10_i64.pow(arg as u32)) % 10;
        match mode {
            MODE_IMMEDIATE => panic!("can't resolve immediate address"),
            MODE_POSITION => value,
            MODE_RELATIVE => value + self.rb as i64,
            _ => unreachable!("invalid mode"),
        }
    }

    #[inline]
    fn fetch(&self, addr: usize) -> i64 {
        self.memory.get(addr).copied().unwrap_or(0)
    }

    #[inline]
    fn op_arity(op: i64) -> usize {
        match op {
            OP_ADDI => 3,
            OP_MULT => 3,
            OP_INPU => 1,
            OP_OUTP => 1,
            OP_JMPT => 2,
            OP_JMPF => 2,
            OP_LESS => 3,
            OP_EQUA => 3,
            OP_ADRB => 1,
            OP_TERM => 0,
            _ => unreachable!("invalid opcode"),
        }
    }

    fn set2(&mut self, addr: i64, value: i64) {
        let addr = addr as usize;
        if self.memory.len() <= addr {
            self.memory.resize(addr + 1, 0);
        }

        self.memory[addr] = value;
    }

    pub fn run(&mut self) -> Result<State> {
        if false {
            return self._run();
        }

        loop {
            let instr = self.fetch(self.ip);
            let op = instr % 100;
            let modes = instr / 100;

            match op {
                OP_ADDI => self.set2(
                    self.addr(modes, 2),
                    self.param(modes, 0) + self.param(modes, 1),
                ),
                OP_MULT => self.set2(
                    self.addr(modes, 2),
                    self.param(modes, 0) * self.param(modes, 1),
                ),
                OP_LESS => self.set2(
                    self.addr(modes, 2),
                    (self.param(modes, 0) < self.param(modes, 1)) as i64,
                ),
                OP_EQUA => self.set2(
                    self.addr(modes, 2),
                    (self.param(modes, 0) == self.param(modes, 1)) as i64,
                ),
                OP_JMPT => {
                    if self.param(modes, 0) != 0 {
                        self.ip = self.param(modes, 1) as usize;
                        continue;
                    }
                }
                OP_JMPF => {
                    if self.param(modes, 0) == 0 {
                        self.ip = self.param(modes, 1) as usize;
                        continue;
                    }
                }
                OP_ADRB => self.rb = (self.rb as i64 + self.param(modes, 0)) as usize,
                OP_INPU => {
                    if let Some(v) = self.inputs.pop_front() {
                        self.set2(self.addr(modes, 0), v);
                    } else {
                        self.ip += 1 + Self::op_arity(op);
                        return Ok(State::AwaitingInput);
                    }
                }
                OP_OUTP => {
                    let v = self.param(modes, 0);
                    self.output = Some(v);
                    self.ip += 1 + Self::op_arity(op);
                    return Ok(State::Suspended(v));
                }
                OP_TERM => {
                    self.ip += 1 + Self::op_arity(op);
                    return Ok(State::Terminated(self.output));
                }
                _ => unreachable!("invalid opcode"),
            };
            self.ip += 1 + Self::op_arity(op);
        }
    }

    pub fn _run(&mut self) -> Result<State> {
        loop {
            let op = self.parse_op()?;
            let result = self.interpret(op)?;
            match result {
                InstrResult::Suspend(x) => return Ok(State::Suspended(x)),
                InstrResult::Terminate => return Ok(State::Terminated(self.output)),
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
            Op::Add(a, b, c) => self.set(c, self.get_value(a) + self.get_value(b))?,
            Op::Multiply(a, b, c) => self.set(c, self.get_value(a) * self.get_value(b))?,
            Op::Less(a, b, c) => self.set(c, (self.get_value(a) < self.get_value(b)) as i64)?,
            Op::Equal(a, b, c) => self.set(c, (self.get_value(a) == self.get_value(b)) as i64)?,
            Op::AdjustRelBase(a) => self.rb = (self.rb as i64 + self.get_value(a)) as usize,
            Op::JumpTrue(a, b) => {
                if self.get_value(a) != 0 {
                    self.ip = self.get_value(b) as usize;
                }
            }
            Op::JumpFalse(a, b) => {
                if self.get_value(a) == 0 {
                    self.ip = self.get_value(b) as usize;
                }
            }
            Op::Input(a) => {
                if let Some(v) = self.inputs.pop_front() {
                    self.set(a, v)?;
                } else {
                    // Push Input operator back so that it gets processed again when the
                    // interpretor is resumed
                    self.ip = self.ip - 2;
                    return Ok(InstrResult::AwaitInput);
                }
            }
            Op::Output(out) => {
                let v = self.get_value(out);
                self.output = Some(v);
                return Ok(InstrResult::Suspend(v));
            }
            Op::Terminate => return Ok(InstrResult::Terminate),
        };
        Ok(InstrResult::Continue)
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
        let v = self
            .memory
            .get(self.ip)
            .ok_or_else(|| anyhow!("expected token"))?;
        self.ip += 1;
        Ok(*v)
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
            Param::Pointer(x) => x,
            Param::Relative(x) => (self.rb as i64 + x) as usize,
            Param::Immediate(_) => {
                return Err(anyhow!("destination can not be an immediate value"))
            }
        };

        if self.memory.len() <= dst {
            self.memory.resize(dst + 1, 0);
        }

        self.memory[dst] = x;

        Ok(())
    }

    pub fn get(&self, addr: usize) -> i64 {
        *self.memory.get(addr).unwrap_or(&0)
    }

    pub fn input(&mut self, v: i64) {
        self.inputs.push_back(v);
    }

    pub fn input_str(&mut self, inputs: &str) {
        for v in inputs.chars().map(|c| c as i64) {
            self.inputs.push_back(v);
        }
    }

    fn parse_instr_addi(&mut self, mut modes: &mut i64) -> Result<Op> {
        Ok(Op::Add(
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
            self.parse_param(&mut modes)?,
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

    #[inline]
    fn parse_param(&mut self, modes: &mut i64) -> Result<Param> {
        let mode = *modes % 10;
        *modes /= 10;

        let p = self.get_token()?;

        Ok(match mode {
            MODE_IMMEDIATE => Param::Immediate(p),
            MODE_POSITION => Param::Pointer(p as usize),
            MODE_RELATIVE => Param::Relative(p),
            _ => return Err(anyhow!("inavlid mode: {}", mode)),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test {
        ($name:ident, $left:expr, $prg:expr) => {
            #[test]
            fn $name() {
                let mut prg = Interpretor::new($prg);
                assert_eq!(Some($left), prg.run_complete().unwrap());
            }
        };
    }

    test!(add, 15, &[01101, 10, 5, 7, 4, 7, 99, 0]);
    test!(add_overflow, 15, &[01101, 10, 5, 7, 4, 7, 99, 0]);
    test!(output_immediate, 10, &[104, 10, 99]);
    test!(output_position, 20, &[004, 3, 99, 20]);
    test!(output_relative, 20, &[109, 5, 204, 0, 99, 20]);
}
