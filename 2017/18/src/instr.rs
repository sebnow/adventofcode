use std::str::FromStr;
use failure::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum Instr {
    Set(char, i32),
    Add(char, i32),
    Mul(char, char),
    Mod(char, i32),
    Snd(char),
    Rcv(char),
    Jgz(char, i32),
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instr = [""; 3];
        for (i, x) in s.split(' ').take(3).enumerate() {
            instr[i] = x;
        }

        let t: &str = instr.get(0).ok_or(format_err!("missing instruction"))?;
        let get_register = |i: usize| -> Result<char, Self::Err> {
            instr
                .get(i)
                .ok_or(format_err!("expecting parameter {} to be a register", i))?
                .chars()
                .next()
                .ok_or(format_err!("expecting parameter {} to be a character", i))
        };
        let get_integral = |i: usize| -> Result<i32, Self::Err> {
            instr
                .get(i)
                .ok_or(format_err!("expecting parameter {}", i))?
                .parse()
                .map_err(|e| {
                    format_err!("expecting parameter {} to be integral: {}", i, e)
                })
        };

        match t {
            "set" => Ok(Instr::Set(get_register(1)?, get_integral(2)?)),
            "add" => Ok(Instr::Add(get_register(1)?, get_integral(2)?)),
            "mul" => Ok(Instr::Mul(get_register(1)?, get_register(2)?)),
            "mod" => Ok(Instr::Mod(get_register(1)?, get_integral(2)?)),
            "snd" => Ok(Instr::Snd(get_register(1)?)),
            "rcv" => Ok(Instr::Rcv(get_register(1)?)),
            "jgz" => Ok(Instr::Jgz(get_register(1)?, get_integral(2)?)),
            _ => Err(format_err!("invalid instruction: {}", t)),
        }
    }
}
