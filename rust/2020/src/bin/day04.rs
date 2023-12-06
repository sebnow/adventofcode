use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::str::FromStr;

struct Byr(u64);
struct Iyr(u64);
struct Eyr(u64);
enum Hgt {
    Cm(u64),
    In(u64),
}
struct Hcl(String);
struct Ecl(String);
struct Pid(u32);

fn parse_num_in_range(s: &str, min: u64, max: u64) -> Result<u64, anyhow::Error> {
    let v = s.parse()?;

    if v >= min && v <= max {
        Ok(v)
    } else {
        Err(anyhow!("out of range"))
    }
}

impl FromStr for Byr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_num_in_range(s, 1920, 2002).map(|v| Byr(v))
    }
}

impl FromStr for Iyr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_num_in_range(s, 2010, 2020).map(|v| Iyr(v))
    }
}

impl FromStr for Eyr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_num_in_range(s, 2020, 2030).map(|v| Eyr(v))
    }
}

impl FromStr for Hgt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len <= 2 {
            return Err(anyhow!("too short"));
        }

        let (v, unit) = s.split_at(len - 2);
        match unit {
            "cm" => Ok(Hgt::Cm(parse_num_in_range(v, 150, 193)?)),
            "in" => Ok(Hgt::In(parse_num_in_range(v, 59, 76)?)),
            _ => Err(anyhow!("invalid unit {}", unit)),
        }
    }
}

impl FromStr for Hcl {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len != 7 {
            return Err(anyhow!("not a colour"));
        }

        let (hash, chars) = s.split_at(1);
        if hash != "#" {
            return Err(anyhow!("missing hash"));
        }

        if !chars
            .chars()
            .all(|c| c.is_numeric() || (c >= 'a' && c <= 'f'))
        {
            return Err(anyhow!("invalid hex"));
        }

        Ok(Hcl(s.to_string()))
    }
}

impl FromStr for Ecl {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(Ecl(s.to_string())),
            _ => Err(anyhow!("not a color {}", s)),
        }
    }
}

impl FromStr for Pid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse()?;
        if s.len() != 9 {
            return Err(anyhow!("not 9 digits"));
        }

        Ok(Pid(v))
    }
}
fn parse_input(s: &str) -> Result<Vec<HashMap<&str, &str>>> {
    let mut passports = Vec::default();
    let mut passport = HashMap::default();

    for l in s.lines() {
        if l == "" {
            let old = passport;
            passport = HashMap::default();
            passports.push(old);
        }

        for pair in l.split_ascii_whitespace() {
            let mut kv = pair.split(":");
            let key = kv.next().ok_or_else(|| anyhow!("key not found"))?;
            let value = kv.next().ok_or_else(|| anyhow!("value not found"))?;

            passport.insert(key, value);
        }
    }

    passports.push(passport);

    Ok(passports)
}

fn part_one(input: &str) -> String {
    parse_input(input)
        .unwrap()
        .iter()
        .filter(|&passport| {
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|&field| passport.contains_key(field))
        })
        .count()
        .to_string()
}

fn part_two(input: &str) -> String {
    parse_input(input)
        .unwrap()
        .iter()
        .filter(|&passport| {
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|&field| {
                    passport.get(field).map_or(false, |&value| match field {
                        "byr" => value.parse::<Byr>().is_ok(),
                        "iyr" => value.parse::<Iyr>().is_ok(),
                        "eyr" => value.parse::<Eyr>().is_ok(),
                        "hgt" => value.parse::<Hgt>().is_ok(),
                        "hcl" => value.parse::<Hcl>().is_ok(),
                        "ecl" => value.parse::<Ecl>().is_ok(),
                        "pid" => value.parse::<Pid>().is_ok(),
                        _ => true,
                    })
                })
        })
        .count()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day04.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 4, 1, 1);
    test_example!(example_two_1, part_two, 4, 2, 1);
    test_example!(example_two_2, part_two, 4, 2, 2);

    #[test]
    fn validation() {
        assert!("2002".parse::<Byr>().is_ok());
        assert!("2003".parse::<Byr>().is_err());

        assert!("60in".parse::<Hgt>().is_ok());
        assert!("190cm".parse::<Hgt>().is_ok());
        assert!("190in".parse::<Hgt>().is_err());
        assert!("190".parse::<Hgt>().is_err());

        assert!("#123abc".parse::<Hcl>().is_ok());
        assert!("#123abz".parse::<Hcl>().is_err());
        assert!("123abc".parse::<Hcl>().is_err());

        assert!("brn".parse::<Ecl>().is_ok());
        assert!("wat".parse::<Ecl>().is_err());

        assert!("000000001".parse::<Pid>().is_ok());
        assert!("0123456789".parse::<Pid>().is_err());
    }
}
