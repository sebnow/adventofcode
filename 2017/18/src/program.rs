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
    pid: i64,
    mem: Memory,
    snd: Sender<i64>,
    rcv: Receiver<i64>,
    msgs_sent: i64,
}

impl Program {
    pub fn new(pid: i64, snd: Sender<i64>, rcv: Receiver<i64>) -> Self {
        let mut mem = Memory::new();
        mem.set('p', pid);
        Program {
            pc: 0,
            pid: pid,
            mem: Memory::new(),
            snd: snd,
            rcv: rcv,
            msgs_sent: 0,
        }
    }

    fn get_value(&self, v: &Value) -> i64 {
        match v {
            &Value::Direct(x) => x,
            &Value::Indirect(r) => self.mem.get(r),
        }
    }

    pub fn msgs_sent(&self) -> i64 {
        self.msgs_sent
    }

    pub fn execute<'a>(&mut self, instructions: &[Instr]) -> Result<State, Error> {
        let mut jmp = 1;
        let instr = instructions
            .get(self.pc)
            .ok_or(format_err!("no instruction at {}", self.pc))?;

        //        println!(
        //            "PID:{} | PC:{:2} | {:20} \t  \t{:?}",
        //            self.pid, self.pc, instr, self.mem
        //        );
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
            &Instr::Jgz(ref v, ref offset) => {
                let x = self.get_value(v);
                if x > 0 {
                    jmp = self.get_value(offset);
                }
            }
            &Instr::Snd(ref r) => {
                if let Err(e) = self.snd.send(self.get_value(r)) {
                    return Err(format_err!("failed to send: {}", e));
                }
                self.msgs_sent += 1;
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
