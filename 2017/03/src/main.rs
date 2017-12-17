#[derive(Debug, Eq, PartialEq, Clone)]
struct Point(i32, i32);

const RIGHT: Point = Point(1, 0);
const UP: Point = Point(0, 1);
const LEFT: Point = Point(-1, 0);
const DOWN: Point = Point(0, -1);

impl Point {
    pub fn manhattan_distance(&self, b: Self) -> u32 {
        let (x1, y1) = (self.0, self.1);
        let (x2, y2) = (b.0, b.1);

        ((x1 - x2).abs() + (y1 - y2).abs()) as u32
    }

    pub fn step(&mut self, delta: &Point) {
        self.0 += delta.0;
        self.1 += delta.1;
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}

impl From<i32> for Point {
    fn from(x: i32) -> Self {
        if x <= 1 {
            return Point(0, 0);
        }

        let mut p = Point(0, 0);
        let mut topright = (0, 0);
        let mut bottomleft = (0, 0);
        let mut dir = &RIGHT;

        for _ in 2..x + 1 {
            p.step(&dir);

            if p.x() > topright.0 {
                dir = &UP;
                topright.0 = p.x();
            } else if p.y() > topright.1 {
                dir = &LEFT;
                topright.1 = p.y();
            } else if p.x() < bottomleft.0 {
                dir = &DOWN;
                bottomleft.0 = p.x();
            } else if p.y() < bottomleft.1 {
                dir = &RIGHT;
                bottomleft.1 = p.y();
            }
        }

        p
    }
}

fn answer_1(input: i32) -> u32 {
    Point::from(input).manhattan_distance(Point(0, 0))
}

fn main() {
    println!("Part 1: {}", answer_1(347991));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(answer_1(1), 0);
        assert_eq!(answer_1(12), 3);
        assert_eq!(answer_1(23), 2);
        assert_eq!(answer_1(1024), 31);
    }

    #[test]
    fn manhattan_distance_symmetrical() {
        let centre = Point(0, 0);
        assert_eq!(
            centre.manhattan_distance(Point(1, 1)),
            centre.manhattan_distance(Point(-1, -1))
        );
        assert_eq!(
            centre.manhattan_distance(Point(1, 2)),
            centre.manhattan_distance(Point(-1, -2))
        );
    }

    #[test]
    fn input_into_point() {
        assert_eq!(Point::from(1), Point(0, 0));
        assert_eq!(Point::from(3), Point(1, 1));
        assert_eq!(Point::from(11), Point(2, 0));
        assert_eq!(Point::from(13), Point(2, 2));
        assert_eq!(Point::from(16), Point(-1, 2));
        assert_eq!(Point::from(17), Point(-2, 2));
        assert_eq!(Point::from(20), Point(-2, -1));
        assert_eq!(Point::from(21), Point(-2, -2));
        assert_eq!(Point::from(23), Point(0, -2));
        assert_eq!(Point::from(25), Point(2, -2));
        assert_eq!(Point::from(26), Point(3, -2));
        assert_eq!(Point::from(45), Point(-1, -3));
    }

}
