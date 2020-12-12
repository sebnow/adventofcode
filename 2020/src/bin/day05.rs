fn seat_id(row: u32, column: u32) -> u32 {
    row * 8 + column
}

fn get_seats<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.lines().map(|line| {
        let res =
            line.chars().fold(
                (0, 127, 0, 7),
                |(row_min, row_max, col_min, col_max), c| match c {
                    'F' => (row_min, (row_min + row_max) / 2, col_min, col_max),
                    'B' => (
                        ((row_min + row_max) as f32 / 2.0).ceil() as u32,
                        row_max,
                        col_min,
                        col_max,
                    ),
                    'L' => (row_min, row_max, col_min, (col_min + col_max) / 2),
                    'R' => (
                        row_min,
                        row_max,
                        ((col_min + col_max) as f32 / 2.0).ceil() as u32,
                        col_max,
                    ),
                    _ => panic!("unknown {}", c),
                },
            );
        seat_id(res.0, res.2)
    })
}

fn part_one(input: &str) -> String {
    get_seats(input).max().unwrap().to_string()
}

fn part_two(input: &str) -> String {
    let mut seats: Vec<u32> = get_seats(input).collect();
    seats.sort();

    seats
        .iter()
        .zip(seats.iter().skip(1))
        .find_map(|(&a, &b)| if a + 2 == b { Some(a + 1) } else { None })
        .unwrap()
        .to_string()
}

fn main() {
    let input = include_str!("../../input/day05.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 5, 1, 1);
    test_example!(example_one_2, part_one, 5, 1, 2);
    test_example!(example_one_3, part_one, 5, 1, 3);
    test_example!(example_one_4, part_one, 5, 1, 4);
}
