use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;

fn get_orbiting<'a>(orbits: &HashMap<&'a str, &'a str>, mut satellite: &'a str) -> Vec<&'a str> {
    let mut orbiting = Vec::new();

    while let Some(&o) = orbits.get(satellite) {
        orbiting.push(o);
        satellite = o;
    }

    orbiting
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| l.split(')').map(|x| x.to_owned()).collect())
        .collect()
}

#[aoc(day6, part1)]
fn answer_1<'a>(input: &'a [Vec<String>]) -> Result<usize> {
    let mut tree: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

    for pairs in input {
        let orbits = tree.entry(&pairs[0]).or_insert_with(Vec::new);
        orbits.push(&pairs[1]);
    }

    let mut to_visit = vec![("COM", 1)];
    let mut total_orbits = 0;
    while let Some((x, count)) = to_visit.pop() {
        if let Some(orbits) = tree.get(x) {
            for o in orbits {
                total_orbits += count;
                to_visit.push((o, count + 1));
            }
        }
    }

    Ok(total_orbits)
}

#[aoc(day6, part2)]
fn answer_2<'a>(input: &'a [Vec<String>]) -> Result<usize> {
    let mut satellites: HashMap<&'a str, &'a str> = HashMap::new();

    input.iter().for_each(|pairs| {
        satellites.insert(&pairs[1], &pairs[0]);
    });

    let you = get_orbiting(&satellites, "YOU");
    let san = get_orbiting(&satellites, "SAN");

    for (i, x) in you.iter().enumerate() {
        for (j, y) in san.iter().enumerate() {
            if x == y {
                return Ok(i + j);
            }
        }
    }

    Err(anyhow!("path not found"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(
            42,
            answer_1(&input_generator(
                r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#
            ))
            .unwrap()
        );
    }
}
