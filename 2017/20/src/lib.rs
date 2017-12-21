#[macro_use]
extern crate nom;

#[macro_use]
extern crate failure;

#[derive(Debug, Eq, PartialEq)]
pub struct Coord(i32, i32, i32);

pub struct Particle {
    position: Coord,
    velocity: Coord,
    acceleration: Coord
}

named!(coord_parser<Coord>,
       chain!(tag!("<") ~ digit ~ tag!(">"))
fn parse_line(input: &str) -> Particle {
    input.split(", ").map(|s| {
        s.chars().skip(3).take_while(|&c| c != '>').split(",").map(|i| i.parse()).collect()
    };

    Particle{
        position: Coord(0, 0, 0),
        velocity: Coord(0, 0, 0),
        acceleration: Coord(0, 0, 0)
    }
}

fn parse_input(input: &str) -> Vec<Particle> {
    input.lines().map(parse_line).collect()
}

pub fn answer_1(input: &str) -> Result<usize, failure::Error> {
    Ok(0)
}

pub fn answer_2(input: &str) -> Result<i64, failure::Error> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = [
            "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>",
            "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
        ].join("\n");

        assert_eq!(Ok(0), answer_1(&input));
    }
}
