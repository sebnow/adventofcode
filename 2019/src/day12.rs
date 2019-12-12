use anyhow::Result;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    fn add_vec(&mut self, v: &Vec3) {
        self.x += v.0[0];
        self.y += v.0[1];
        self.z += v.0[2];
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3([i64; 3]);

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Moon {
    pos: Point3D,
    vel: Vec3,
}

impl Moon {
    fn potential_energy(&self) -> i64 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel.0[0].abs() + self.vel.0[1].abs() + self.vel.0[2].abs()
    }

    fn energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn map_moons(ps: &[Point3D]) -> Vec<Moon> {
    ps.iter()
        .map(|&p| Moon {
            pos: p,
            vel: Vec3([0, 0, 0]),
        })
        .collect()
}

fn get_gravity(moons: &[Moon], moon: &Moon) -> Vec3 {
    let x = moons.iter().filter(|m| moon.pos.x < m.pos.x).count() as i64
        - moons.iter().filter(|m| moon.pos.x > m.pos.x).count() as i64;
    let y: i64 = moons.iter().filter(|m| moon.pos.y < m.pos.y).count() as i64
        - moons.iter().filter(|m| moon.pos.y > m.pos.y).count() as i64;
    let z = moons.iter().filter(|m| moon.pos.z < m.pos.z).count() as i64
        - moons.iter().filter(|m| moon.pos.z > m.pos.z).count() as i64;

    Vec3([x, y, z])
}

fn tick(moons: &mut Vec<Moon>) {
    let prev_moons = moons.to_owned();
    for moon in moons.iter_mut() {
        let g = get_gravity(&prev_moons, moon);
        moon.vel = moon.vel + g;
        moon.pos.add_vec(&moon.vel);
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Point3D> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    input
        .lines()
        .map(|l| {
            let c = re.captures(l).unwrap();
            Point3D {
                x: c.get(1).unwrap().as_str().parse().unwrap(),
                y: c.get(2).unwrap().as_str().parse().unwrap(),
                z: c.get(3).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn answer_1(input: &[Point3D]) -> Result<i64> {
    let mut moons = map_moons(input);

    for _ in 0..1_000 {
        tick(&mut moons);
    }

    Ok(moons.iter().map(|m| m.energy()).sum())
}

#[aoc(day12, part2)]
fn answer_2(_input: &[Point3D]) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add_vec() {
        let mut p = Point3D { x: 1, y: 2, z: 3 };
        p.add_vec(&Vec3([-2, 0, 3]));
        assert_eq!(p, Point3D { x: -1, y: 2, z: 6 });
    }

    #[test]
    fn test_energy() {
        let moon = Moon {
            pos: Point3D { x: 2, y: 1, z: 3 },
            vel: Vec3([3, 2, 1]),
        };

        assert_eq!(6, moon.potential_energy());
        assert_eq!(6, moon.kinetic_energy());
        assert_eq!(36, moon.energy());

        let moon = Moon {
            pos: Point3D { x: 1, y: 8, z: 0 },
            vel: Vec3([1, 1, 3]),
        };

        assert_eq!(9, moon.potential_energy());
        assert_eq!(5, moon.kinetic_energy());
        assert_eq!(45, moon.energy());
    }

    #[test]
    fn test_example_1_1() {
        let mut moons = map_moons(&input_generator(
            r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        ));

        assert_eq!(
            moons,
            vec![
                Moon {
                    pos: Point3D { x: -1, y: 0, z: 2 },
                    vel: Vec3([0, 0, 0]),
                },
                Moon {
                    pos: Point3D {
                        x: 2,
                        y: -10,
                        z: -7
                    },
                    vel: Vec3([0, 0, 0]),
                },
                Moon {
                    pos: Point3D { x: 4, y: -8, z: 8 },
                    vel: Vec3([0, 0, 0]),
                },
                Moon {
                    pos: Point3D { x: 3, y: 5, z: -1 },
                    vel: Vec3([0, 0, 0]),
                },
            ]
        );

        tick(&mut moons);
        assert_eq!(
            moons,
            vec![
                Moon {
                    pos: Point3D { x: 2, y: -1, z: 1 },
                    vel: Vec3([3, -1, -1]),
                },
                Moon {
                    pos: Point3D { x: 3, y: -7, z: -4 },
                    vel: Vec3([1, 3, 3]),
                },
                Moon {
                    pos: Point3D { x: 1, y: -7, z: 5 },
                    vel: Vec3([-3, 1, -3]),
                },
                Moon {
                    pos: Point3D { x: 2, y: 2, z: 0 },
                    vel: Vec3([-1, -3, 1]),
                },
            ]
        );

        tick(&mut moons);
        assert_eq!(
            moons,
            vec![
                Moon {
                    pos: Point3D { x: 5, y: -3, z: -1 },
                    vel: Vec3([3, -2, -2]),
                },
                Moon {
                    pos: Point3D { x: 1, y: -2, z: 2 },
                    vel: Vec3([-2, 5, 6]),
                },
                Moon {
                    pos: Point3D { x: 1, y: -4, z: -1 },
                    vel: Vec3([0, 3, -6]),
                },
                Moon {
                    pos: Point3D { x: 1, y: -4, z: 2 },
                    vel: Vec3([-1, -6, 2]),
                },
            ]
        );
    }

    #[test]
    fn test_example_1_2() {
        let mut moons = map_moons(&input_generator(
            r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#,
        ));

        for _ in 0..100 {
            tick(&mut moons);
        }

        assert_eq!(1940 as i64, moons.iter().map(|m| m.energy()).sum());
    }
}
