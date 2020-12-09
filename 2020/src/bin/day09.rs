use anyhow::{anyhow, Result};
fn parse_input(s: &str) -> Result<Vec<i64>> {
    s.lines()
        .map(|l| l.parse().map_err(|err| anyhow!("{}", err)))
        .collect()
}

fn part_one(input: &str) -> String {
    let window_size = 25;
    let nums = parse_input(input).unwrap();

    for (offset, &x) in nums.iter().skip(window_size).enumerate() {
        let prev = &nums[offset..(offset + window_size)];

        let mut found = false;
        'outer: for a in prev {
            for b in prev {
                if a + b == x {
                    found = true;
                    break 'outer;
                }
            }
        }

        if !found {
            return x.to_string();
        }
    }

    "".to_string()
}

fn part_two(input: &str) -> String {
    let target: i64 = part_one(input).parse().unwrap();
    let nums = parse_input(input).unwrap();

    for (offset, _) in nums.iter().enumerate() {
        let mut size = 2;

        loop {
            let set = &nums[offset..offset + size];
            let sum: i64 = set.iter().sum();

            if sum == target {
                return (set.iter().min().unwrap() + set.iter().max().unwrap()).to_string();
            }

            if sum > target {
                break;
            }

            size += 1;
        }
    }

    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day09.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    //    test_example!(example_one_1, part_one, 3, 1, 1);
    //    test_example!(example_two_1, part_two, 3, 2, 1);
}
