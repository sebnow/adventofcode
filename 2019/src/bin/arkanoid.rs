use anyhow::Result;
use std::time::Duration;

use adventofcode2019::arkanoid;

fn get_rom(path: &str) -> Result<Vec<i64>> {
    let buffer = std::fs::read_to_string(path)?;

    Ok(buffer
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect())
}

fn main() -> Result<()> {
    let mut rom = get_rom("input/2019/day13.txt")?;
    rom[0] = 2;

    let mut game = arkanoid::Game::new(&rom);

    while !game.is_initialized() {
        game.update()?;
    }

    while !game.is_over() {
        game.update()?;

        let g = game.get_grid();
        println!("{}", g);

        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
