use anyhow::{anyhow, Result};

fn get_input() -> Result<Vec<u32>> {
    include_str!("../../input/day01.txt")
        .lines()
        .map(|l| l.parse().map_err(|e| anyhow!("failed to parse {}", e)))
        .collect()
}

fn part_one(input: &[u32]) -> Result<String> {
    for a in input {
        for b in input {
            if a+b == 2020 {
                return Ok(format!("{}", a * b))
            }
        }
    }

    Err(anyhow!("answer not found"))
}

fn part_two(input: &[u32]) -> Result<String>{
    for a in input {
        for b in input {
            for c in input {
                if a+b+c == 2020 {
                    return Ok(format!("{}", a * b * c))
                }
            }
        }
    }

    Err(anyhow!("answer not found"))
}

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part one: {}", part_one(&input).unwrap());
    println!("Part two: {}", part_two(&input).unwrap());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!("514579", part_one(&vec![ 1721, 979, 366, 299, 675, 1456 ]).unwrap())
    }

    #[test]
    fn examples_2() {
        assert_eq!("241861950", part_two(&vec![ 1721, 979, 366, 299, 675, 1456 ]).unwrap())
    }
}

