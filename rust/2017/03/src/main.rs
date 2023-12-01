mod point;
mod sum_spiral;

use point::Point;
use sum_spiral::Spiral;

pub fn spiral_point(x: i32) -> Point {
    if x <= 1 {
        return Point::new(0, 0);
    }

    let mut p = Point::new(0, 0);
    let mut topright = (0, 0);
    let mut bottomleft = (0, 0);
    let mut dir = &point::RIGHT;

    for _ in 2..x + 1 {
        p.step(&dir);

        if p.x() > topright.0 {
            dir = &point::UP;
            topright.0 = p.x();
        } else if p.y() > topright.1 {
            dir = &point::LEFT;
            topright.1 = p.y();
        } else if p.x() < bottomleft.0 {
            dir = &point::DOWN;
            bottomleft.0 = p.x();
        } else if p.y() < bottomleft.1 {
            dir = &point::RIGHT;
            bottomleft.1 = p.y();
        }
    }

    p
}

fn answer_1(input: i32) -> u32 {
    spiral_point(input).manhattan_distance(Point::new(0, 0))
}

fn answer_2(input: i32) -> i32 {
    Spiral::new().into_iter().find(|&x| x > input).unwrap()
}

fn main() {
    println!("Part 1: {}", answer_1(347991));
    println!("Part 2: {}", answer_2(347991));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(answer_1(1), 0);
        assert_eq!(answer_1(12), 3);
        assert_eq!(answer_1(23), 2);
        assert_eq!(answer_1(1024), 31);
    }

    #[test]
    fn example_2() {
        assert_eq!(answer_2(5), 10);
        assert_eq!(answer_2(360), 362);
        assert_eq!(answer_2(600), 747);
        assert_eq!(answer_2(700), 747);
        assert_eq!(answer_2(800), 806);
    }

    #[test]
    fn input_into_point() {
        assert_eq!(spiral_point(1), Point::new(0, 0));
        assert_eq!(spiral_point(3), Point::new(1, 1));
        assert_eq!(spiral_point(11), Point::new(2, 0));
        assert_eq!(spiral_point(13), Point::new(2, 2));
        assert_eq!(spiral_point(16), Point::new(-1, 2));
        assert_eq!(spiral_point(17), Point::new(-2, 2));
        assert_eq!(spiral_point(20), Point::new(-2, -1));
        assert_eq!(spiral_point(21), Point::new(-2, -2));
        assert_eq!(spiral_point(23), Point::new(0, -2));
        assert_eq!(spiral_point(25), Point::new(2, -2));
        assert_eq!(spiral_point(26), Point::new(3, -2));
        assert_eq!(spiral_point(45), Point::new(-1, -3));
    }
}
