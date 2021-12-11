#[derive(Debug)]
struct Fish {
    days_to_spawn: u32,
}

fn parse_input(s: &str) -> impl Iterator<Item = Fish> + '_ {
    s.trim().split(',').map(|x| Fish {
        days_to_spawn: x.parse().unwrap(),
    })
}

fn breed_for_days(s: &str, n: usize) -> usize {
    let mut fishies = [0; 9];

    for f in parse_input(s) {
        fishies[f.days_to_spawn as usize] += 1;
    }

    for _ in 0..n {
        let baby_fishies = fishies[0];
        for i in 1..fishies.len() {
            fishies[i - 1] = fishies[i];
        }

        fishies[6] += baby_fishies;
        fishies[8] = baby_fishies;
    }

    fishies.iter().sum()
}

fn part_one(s: &str) -> String {
    format!("{}", breed_for_days(s, 80))
}

fn part_two(s: &str) -> String {
    format!("{}", breed_for_days(s, 256))
}

fn main() {
    let input = include_str!("../../input/day06.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_6_1, part_one, 6, 1, 1);
    test_example!(example_6_2, part_two, 6, 2, 1);
}
