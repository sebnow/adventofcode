#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point(i32, i32);

pub const RIGHT: Point = Point(1, 0);
pub const UP: Point = Point(0, 1);
pub const LEFT: Point = Point(-1, 0);
pub const DOWN: Point = Point(0, -1);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }

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

#[cfg(test)]
mod test {
    use super::*;

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
}
