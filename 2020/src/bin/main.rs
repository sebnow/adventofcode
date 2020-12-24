extern crate adventofcode2020;

use aocutil::AdventOfCode;
use clap::{App, Arg};

fn register() -> AdventOfCode {
    let mut aoc = AdventOfCode::new();
    adventofcode2020::day01::register(&mut aoc);
    aoc
}

fn main() {
    let matches = App::new("Advent Of Code 2020")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .takes_value(true),
        )
        .get_matches();

    let aoc = register();
    let day: u32 = matches.value_of("day").unwrap().parse().unwrap();
    aoc.run_day(2020, day)
}
