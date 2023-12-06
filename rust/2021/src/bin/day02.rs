#[derive(Default)]
struct Pos {
    d: i32,
    y: i32,
    aim: i32,
}

fn part_one(s: &str) -> String {
    let pos = s.lines().fold(Pos::default(), |p, l| {
        let parts: Vec<_> = l.split(' ').collect();
        let units: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "forward" => Pos{y: p.y + units, ..p},
            "down" => Pos{d: p.d + units, ..p},
            "up" => Pos{d: p.d - units, ..p},
            _ => panic!("invalid direction {}", parts[0]),
        }
    });

    format!("{}", pos.y * pos.d)
}

fn part_two(s: &str) -> String {
    let pos = s.lines().fold(Pos::default(), |p, l| {
        let parts: Vec<_> = l.split(' ').collect();
        let units: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "forward" => Pos{y: p.y + units, d: p.d + p.aim * units, ..p},
            "down" => Pos{aim: p.aim + units, ..p},
            "up" => Pos{aim: p.aim - units, ..p},
            _ => panic!("invalid direction {}", parts[0]),
        }
    });

    format!("{}", pos.y * pos.d)
}

fn main() {
    let input = include_str!("../../../../input/2021/day02.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_2_1, part_one, 2, 1, 1);
    test_example!(example_2_2, part_two, 2, 2, 1);
}
