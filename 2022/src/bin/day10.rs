use anyhow::Result;

enum Instr {
    Noop,
    AddX(i64),
}

fn parse_input(s: &str) -> Result<impl Iterator<Item = Instr> + '_> {
    Ok(s.lines().flat_map(|l| -> Result<Instr> {
        Ok(match l.split_once(' ') {
            None => Instr::Noop,
            Some((_, n)) => Instr::AddX(n.parse()?),
        })
    }))
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let cycles = input
        .fold((vec![1], 1), |(mut cycles, mut x), instr| match instr {
            Instr::Noop => {
                cycles.push(x);
                (cycles, x)
            }
            Instr::AddX(n) => {
                cycles.push(x);
                x += n;
                cycles.push(x);
                (cycles, x)
            }
        })
        .0;

    let signal_strength: i64 = 20 * cycles[19]
        + cycles
            .iter()
            .enumerate()
            .skip(59)
            .step_by(40)
            .map(|(cycle, x)| (cycle as i64 + 1) * x)
            .sum::<i64>();

    signal_strength.to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day10.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(day10_1, part_one, 10, 1, 1);
    //test_example!(day10_2, part_two, 10, 2, 1);
}
