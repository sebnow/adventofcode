use std::sync::mpsc::{Receiver, Sender};
use failure::Error;
use memory::*;
use instr::*;

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Running,
    Completed,
}

pub struct Program {
    pc: usize,
    mem: Memory,
    snd: Sender<i64>,
    rcv: Receiver<i64>,
}

impl Program {
    pub fn new(snd: Sender<i64>, rcv: Receiver<i64>) -> Self {
        Program {
            pc: 0,
            mem: Memory::new(),
            snd: snd,
            rcv: rcv,
        }
    }

    fn get_value(&self, v: &Value) -> i64 {
        match v {
            &Value::Direct(x) => x,
            &Value::Indirect(r) => self.mem.get(r),
        }
    }

    pub fn execute<'a>(&mut self, instructions: &[Instr]) -> Result<State, Error> {
        let mut jmp = 1;
        let instr = instructions
            .get(self.pc)
            .ok_or(format_err!("no instruction at {}", self.pc))?;

        println!("{} {:?}", self.pc, instr);
        match instr {
            &Instr::Set(r, ref v) => {
                let x = self.get_value(v);
                self.mem.set(r, x);
            }
            &Instr::Add(r, ref v) => {
                let new = self.mem.get(r) + self.get_value(v);
                self.mem.set(r, new);
            }
            &Instr::Mul(r, ref v) => {
                let new = self.mem.get(r) * self.get_value(v);
                self.mem.set(r, new);
            }
            &Instr::Mod(r, ref v) => {
                let new = self.mem.get(r) % self.get_value(v);
                self.mem.set(r, new);
            }
            &Instr::Jgz(r, ref offset) => {
                let v = self.mem.get(r);
                if v > 0 {
                    jmp = self.get_value(offset);
                }
            }
            &Instr::Snd(r) => {
                if let Err(e) = self.snd.send(self.mem.get(r)) {
                    return Err(format_err!("failed to send: {}", e));
                }
            }
            &Instr::Rcv(r) => {
                match self.rcv.recv() {
                    Ok(x) => self.mem.set(r, x),
                    Err(e) => return Err(format_err!("err recv: {}", e)),
                };
            }
        };

        let new_pc = self.pc as i64 + jmp;
        if 0 <= new_pc && (new_pc as usize) < instructions.len() {
            self.pc = new_pc as usize;
            Ok(State::Running)
        } else {
            Ok(State::Completed)
        }
    }
}
