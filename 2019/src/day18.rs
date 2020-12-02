use crate::grid::{Collision, Grid, Point, MASK_CROSSHAIR};
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Wall,
    Space,
    Key(char),
    Door(char),
    Entrance,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Space
    }
}

impl std::str::FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Tile::*;
        let c = s
            .chars()
            .nth(0)
            .ok_or_else(|| anyhow!("missing character"))?;

        match c {
            '#' => Ok(Wall),
            '.' => Ok(Space),
            'a'..='z' => Ok(Key(c)),
            'A'..='Z' => Ok(Door(c)),
            '@' => Ok(Entrance),
            _ => Err(anyhow!("invalid tile '{}'", c)),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tile::*;

        write!(
            f,
            "{}",
            match *self {
                Wall => '#',
                Space => '.',
                Key(c) => c,
                Door(c) => c,
                Entrance => '@',
            }
        )
    }
}

impl Collision for Tile {
    fn is_collidable(&self) -> bool {
        use Tile::*;

        match self {
            Wall => true,
            Door(_) => true,
            Key(_) => false,
            Space => false,
            Entrance => false,
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|x| x.to_string().parse().unwrap()).collect())
        .collect();

    Grid::from_vec2d(tiles)
}

#[derive(Debug)]
struct Candidate<'a> {
    prev: Point,
    position: Point,
    keys_to_collect: Vec<(&'a Point, char)>,
    keys: u32,
    steps: usize,

    // For debugging
    // TODO: Remove
    history: Vec<char>,
}

impl<'a> Candidate<'a> {
    fn add_key(&mut self, key: char) -> Result<()> {
        self.keys = self.keys | 2 << key as u8 - b'a';
        let idx = self
            .keys_to_collect
            .binary_search_by_key(&key, |&(_, k)| k)
            .map_err(|err| anyhow!("key already collected ({})", err))?;
        self.keys_to_collect.remove(idx);
        self.history.push(key);
        Ok(())
    }

    fn has_key_for_door(&self, door: char) -> bool {
        self.has_key(door.to_ascii_lowercase())
    }

    fn has_key(&self, key: char) -> bool {
        self.keys & 2 << key as u8 - b'a' != 0
    }
}

#[aoc(day18, part1)]
fn answer_1(grid: &Grid<Tile>) -> Result<usize> {
    let mut candidates = VecDeque::new();

    let mut complete_candidates: Vec<Candidate> = Vec::new();
    {
        let entrance = grid
            .iter()
            .find_map(|(&p, &t)| match t {
                Tile::Entrance => Some(p),
                _ => None,
            })
            .ok_or_else(|| anyhow!("entrance not found"))?;
        let mut keys: Vec<_> = grid
            .iter()
            .filter_map(|(p, &t)| match t {
                Tile::Key(k) => Some((p, k)),
                _ => None,
            })
            .collect();
        keys.sort_by_key(|&(_, k)| k);
        assert_ne!(0, keys.len());

        candidates.push_back(Candidate {
            prev: entrance,
            position: entrance,
            keys_to_collect: keys,
            keys: 0,
            steps: 0,
            history: vec![],
        });
    }

    let expected = vec![
        'a', 'f', 'b', 'j', 'g', 'n', 'h', 'd', 'l', 'o', 'e', 'p', 'c', 'i', 'k', 'm',
    ];
    let mut iterations = 0;
    while let Some(mut candidate) = candidates.pop_back() {
        // Infinite loop failsafe
        // TODO: Remove
        //        if iterations > 30 {
        //            break;
        //        }
        //if candidate.steps > 140 {
        //    continue;
        //}

        //let want: Vec<char> = expected
        //    .iter()
        //    .take(candidate.history.len())
        //    .map(|x| *x)
        //    .collect();
        //if candidate.history != want {
        //    continue;
        //}

        // TODO: REMOVE
        {
            println!("");
            println!("=========== {} ==========", iterations);
            println!("Prev: {:?}", candidate.prev);
            println!("Pos: {:?}", candidate.position);
            println!("Path: {:?}", candidate.history);
            println!("Left: {:?}", candidate.keys_to_collect);
            println!("Steps: {}", candidate.steps);
            let mut g = grid.clone();
            g.insert(candidate.position, Tile::Entrance);
            println!("{}", g);
        }

        let tile = grid
            .get(&candidate.position)
            .ok_or_else(|| anyhow!("off the beaten path"))?;

        if let Tile::Key(k) = tile {
            if !candidate.has_key(*k) {
                println!("Picking up {}", k);
                candidate.add_key(*k)?;
                if candidate.keys_to_collect.len() == 0 {
                    complete_candidates.push(candidate);
                    continue;
                }

                // Backtrack in case a new path opened up.
                println!("Backtracking candidate {:?}", candidate.prev);
                candidates.push_back(Candidate {
                    prev: candidate.position,
                    position: candidate.prev,
                    keys_to_collect: candidate.keys_to_collect.clone(),
                    history: candidate.history.clone(),
                    steps: candidate.steps + 1,
                    ..candidate
                });
            } else {
                println!("Skipping {}", k);
            }
        }

        grid.surrounding(&candidate.position, MASK_CROSSHAIR)
            .iter()
            .filter(|(sp, &st)| {
                if *sp == candidate.prev {
                    return false;
                }
                match st {
                    Tile::Door(d) => candidate.has_key_for_door(d),
                    Tile::Wall => false,
                    _ => true,
                }
            })
            .for_each(|&(sp, _)| {
                let c = Candidate {
                    prev: candidate.position,
                    position: sp,
                    keys_to_collect: candidate.keys_to_collect.clone(),
                    history: candidate.history.clone(),
                    steps: candidate.steps + 1,
                    ..candidate
                };
                let dir = candidate.position - candidate.prev;
                println!("Adding candidate {:?}", sp);
                if candidate.position + dir == sp {
                    candidates.push_back(c);
                } else {
                    candidates.push_front(c);
                }
            });

        iterations += 1;
    }

    complete_candidates
        .iter()
        .min_by_key(|c| c.steps)
        .map(|c| {
            println!("winner: {:?}", c);
            //c.steps
            0
        })
        .ok_or_else(|| anyhow!("no paths found"))
}

#[aoc(day18, part2)]
fn answer_2(input: &Grid<Tile>) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        assert_eq!(
            132,
            answer_1(&input_generator(&include_str!("../examples/day18-1-1.txt"))).unwrap()
        );
    }

    #[test]
    fn test_1_2() {
        assert_eq!(
            136,
            answer_1(&input_generator(&include_str!("../examples/day18-1-2.txt"))).unwrap()
        );
    }

    #[test]
    fn test_1_3() {
        assert_eq!(
            81,
            answer_1(&input_generator(&include_str!("../examples/day18-1-3.txt"))).unwrap()
        );
    }
}
