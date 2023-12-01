use std::collections::HashMap;

use point::{Point, DOWN, LEFT, RIGHT, UP};

pub struct Spiral(HashMap<Point, i32>);

impl Spiral {
    pub fn new() -> Self {
        let mut seen = HashMap::new();
        seen.insert(Point::new(0, 0), 1);

        Spiral(seen)
    }

    pub fn get(&mut self, p: Point) -> i32 {
        if let Some(&seen) = self.0.get(&p) {
            return seen;
        }

        let mut sum = 0;
        sum += *self.0.get(&Point::new(p.x() + 1, p.y())).unwrap_or(&0);
        sum += *self.0.get(&Point::new(p.x() + 1, p.y() + 1)).unwrap_or(&0);
        sum += *self.0.get(&Point::new(p.x(), p.y() + 1)).unwrap_or(&0);
        sum += *self.0.get(&Point::new(p.x() - 1, p.y() + 1)).unwrap_or(&0);
        sum += *self.0.get(&Point::new(p.x() - 1, p.y())).unwrap_or(&0);
        sum += *self.0.get(&Point::new(p.x() - 1, p.y() - 1)).unwrap_or(&0);
        sum += *self.0.get(&Point::new(p.x(), p.y() - 1)).unwrap_or(&0);
        sum += *self.0.get(&Point::new(p.x() + 1, p.y() - 1)).unwrap_or(&0);
        self.0.insert(p, sum);

        sum
    }
}

impl IntoIterator for Spiral {
    type Item = i32;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct Iter {
    spiral: Spiral,
    curr: Point,
    bottomleft: (i32, i32),
    topright: (i32, i32),
    dir: Point,
}

impl Iter {
    fn new(spiral: Spiral) -> Self {
        Iter {
            spiral: spiral,
            curr: Point::new(0, 0),
            topright: (0, 0),
            bottomleft: (0, 0),
            dir: RIGHT,
        }
    }
}

impl Iterator for Iter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.curr.step(&self.dir);

        if self.curr.x() > self.topright.0 {
            self.dir = UP;
            self.topright.0 = self.curr.x();
        } else if self.curr.y() > self.topright.1 {
            self.dir = LEFT;
            self.topright.1 = self.curr.y();
        } else if self.curr.x() < self.bottomleft.0 {
            self.dir = DOWN;
            self.bottomleft.0 = self.curr.x();
        } else if self.curr.y() < self.bottomleft.1 {
            self.dir = RIGHT;
            self.bottomleft.1 = self.curr.y();
        }

        Some(self.spiral.get(self.curr.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spiral_get() {
        let mut spiral = Spiral::new();
        assert_eq!(1, spiral.get(Point::new(0, 0)));
        assert_eq!(1, spiral.get(Point::new(1, 0)));
        assert_eq!(2, spiral.get(Point::new(1, 1)));
        assert_eq!(4, spiral.get(Point::new(0, 1)));
        assert_eq!(5, spiral.get(Point::new(-1, 1)));
        assert_eq!(10, spiral.get(Point::new(-1, 0)));
        assert_eq!(11, spiral.get(Point::new(-1, -1)));
        assert_eq!(23, spiral.get(Point::new(0, -1)));
        assert_eq!(25, spiral.get(Point::new(1, -1)));
        assert_eq!(26, spiral.get(Point::new(2, -1)));
    }
}
