use std::str::FromStr;
use failure::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Indirect(char),
    Direct(i64),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(|x| Value::Direct(x)).or_else(|_| {
            s.chars()
                .next()
                .ok_or(format_err!("expecting integral or char"))
                .map(|c| Value::Indirect(c))
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instr {
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Snd(char),
    Rcv(char),
    Jgz(char, Value),
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instr = [""; 3];
        for (i, x) in s.split(' ').take(3).enumerate() {
            instr[i] = x;
        }

        let t: &str = instr.get(0).ok_or(format_err!("missing instruction"))?;
        let get_register = |t: &str, i: usize| -> Result<char, Self::Err> {
            instr
                .get(i)
                .ok_or(format_err!(
                    "{}: expecting parameter {} to be a register",
                    t,
                    i
                ))?
                .chars()
                .next()
                .ok_or(format_err!(
                    "{}: expecting parameter {} to be a character",
                    t,
                    i
                ))
        };
        let get_value = |t: &str, i: usize| -> Result<Value, Self::Err> {
            instr
                .get(i)
                .ok_or(format_err!("{}: expecting parameter {}", t, i))?
                .parse()
                .map_err(|e| format_err!("{}: expecting parameter {} to be a value: {}", t, i, e))
        };

        match t {
            "set" => Ok(Instr::Set(get_register(t, 1)?, get_value(t, 2)?)),
            "add" => Ok(Instr::Add(get_register(t, 1)?, get_value(t, 2)?)),
            "mul" => Ok(Instr::Mul(get_register(t, 1)?, get_value(t, 2)?)),
            "mod" => Ok(Instr::Mod(get_register(t, 1)?, get_value(t, 2)?)),
            "snd" => Ok(Instr::Snd(get_register(t, 1)?)),
            "rcv" => Ok(Instr::Rcv(get_register(t, 1)?)),
            "jgz" => Ok(Instr::Jgz(get_register(t, 1)?, get_value(t, 2)?)),
            _ => Err(format_err!("invalid instruction: {}", t)),
        }
    }
}
