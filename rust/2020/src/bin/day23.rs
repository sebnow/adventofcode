fn find_dest(cups: &[usize], cur: usize, ignore: &[usize]) -> usize {
    let mut d = cur;
    loop {
        d = if d > 1 { d - 1 } else { cups.len() - 1 };
        if !ignore.contains(&d) {
            return d;
        }
    }
}

fn play_game(input: &str, moves: usize, pad: usize) -> Vec<usize> {
    // Create a linked list with each element pointing to the next cup label (i.e. [2, 0, 1]
    // indicates the first cup links to the third cup, the third cup links to the second cup, and
    // the second cups links to the first).
    //
    // This abuses the fact that the cups are sequential (albeit 1-indexed) and thus can be
    // represented in contiguous memory.
    let (mut cups, mut c): (Vec<usize>, usize) = {
        let mut v = vec![0; pad + 1];
        let mut cups: Vec<usize> = parse_input(input).chain(10..=pad).collect();
        cups.push(cups[0]);
        for (&cur, &next) in cups.iter().zip(cups.iter().skip(1)) {
            v[cur] = next;
        }
        (v, cups[0])
    };

    for _ in 1..=moves {
        let next = |n| cups[n];
        let r = [next(c), next(next(c)), next(next(next(c)))];
        // Remove the cups
        //    /-r[0]->-r[1]->-r[2]-\
        // c ------------------------> X
        cups[c] = cups[r[2]];
        let d = find_dest(&cups, c, &r);
        // Add the cups back in at destination
        //     /-r[0]->-r[1]->-r[2]-\
        // d -xxxxxxxxxxxxxxxxxxxxxxxx-> X
        cups[r[2]] = cups[d];
        cups[d] = r[0];
        // Select the new cup
        c = cups[c];
    }

    cups
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
}

fn part_one(input: &str) -> String {
    let cups = play_game(input, 100, input.chars().count() - 1);

    let mut output: Vec<String> = Vec::new();
    let mut n = cups[1];
    while n != 1 {
        output.push(format!("{}", n));
        n = cups[n];
    }

    output.join("")
}

fn part_two(input: &str) -> String {
    let cups = play_game(input, 10_000_000, 1_000_000);

    let a = cups[1];
    let b = cups[a];
    (a * b).to_string()
}

fn main() {
    let input = include_str!("../../../../input/2020/day23.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 23, 1, 1);
    test_example!(example_two_1, part_two, 23, 2, 1);
}
