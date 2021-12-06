#[derive(Debug)]
struct Fish {
    days_to_spawn: u32,
}

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item = Fish> + 'a {
    s.trim().split(",").map(|x| {
        Fish {
            days_to_spawn: x.parse().unwrap(),
        }
    })
}

fn part_one(s: &str) -> String {
    let mut fishies: Vec<_> = parse_input(s).collect();

    for _ in 0..80 {
        for i in 0..fishies.len() {
            if fishies[i].days_to_spawn == 0 {
                fishies.push(Fish { days_to_spawn: 8 });
                fishies[i].days_to_spawn = 7;
            }

            fishies[i].days_to_spawn -= 1;
        }
    }

    format!("{}", fishies.len())
}

fn part_two(s: &str) -> String {
    let mut fishies = [0 as usize; 9];

    for f in parse_input(s) {
        fishies[f.days_to_spawn as usize] += 1;
    }

    for _ in 0..256 {
        let baby_fishies = fishies[0];
        for i in 1..fishies.len() {
            fishies[i - 1] = fishies[i];
        }

        fishies[6] += baby_fishies;
        fishies[8] = baby_fishies;
    }

    format!("{}", fishies.iter().sum::<usize>())
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
