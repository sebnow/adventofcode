use std::str::FromStr;
use failure;

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Move {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let cmd = chars.nth(0).ok_or(format_err!("missing move symbol"))?;
        match cmd {
            's' => {
                let a = chars.collect::<String>().parse()?;
                Ok(Move::Spin(a))
            }
            'x' => {
                let a = chars.take_while(|&x| x != '/').collect::<String>().parse()?;
                let b = s.split('/')
                    .nth(1)
                    .ok_or(format_err!("missing second parameter"))?
                    .parse()?;
                Ok(Move::Exchange(a, b))
            }
            'p' => {
                let a = chars.nth(0).ok_or(format_err!("missing first parameter"))?;
                let b = chars.nth(1).ok_or(format_err!("missing second parameter"))?;
                Ok(Move::Partner(a, b))
            }
            _ => Err(format_err!("incorrect move symbol")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_spin() {
        assert_eq!(
            Ok(Move::Spin(42)),
            Move::from_str("s42").map_err(|x| format!("{}", x))
        );
    }

    #[test]
    fn parse_exchange() {
        assert_eq!(
            Ok(Move::Exchange(4, 2)),
            Move::from_str("x4/2").map_err(|x| format!("{}", x))
        );
    }

    #[test]
    fn parse_partner() {
        assert_eq!(
            Ok(Move::Partner('a', 'b')),
            Move::from_str("pa/b").map_err(|x| format!("{}", x))
        );
    }
}
