#[derive(Debug, Eq, PartialEq)]
struct Answer {
    steps: i32,
}

#[derive(Debug, Eq, PartialEq)]
enum Side {
    Right,
    Top,
    Left,
    Bottom,
}

#[derive(Debug, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    pub fn manhattan_distance(&self, b: Self) -> i32 {
        let (x1, y1) = (self.0, self.1);
        let (x2, y2) = (b.0, b.1);

        (x1 - x2).abs() + (y1 - y2).abs()
    }
}

impl From<i32> for Point {
    fn from(x: i32) -> Self {
        let closest_integral = (x as f32).sqrt().ceil();
        let is_top_right = (closest_integral as i32) % 2 == 0;

        let grid_size = if is_top_right { closest_integral + 1.0 } else { closest_integral } as i32;
        let grid_max = grid_size * grid_size;

        let side = if x > grid_max - grid_size { Side::Bottom }
            else if x > grid_max - (grid_size * 2) + 1 { Side::Left }
            else if x > grid_max - (grid_size * 3) + 2 { Side::Top }
            else  { Side::Right };

        let ring = (grid_size as f32).sqrt().ceil() as i32;
        match side {
            Side::Right => Point(ring, 0),
            Side::Top => Point(0, ring),
            Side::Left => Point(0-ring, 0),
            Side::Bottom => Point(0, 0-ring),
        }
    }
}

fn answer(input: i32) -> Answer {
    let access_point = Point(0, 0);
    Answer{
        steps: Point::from(input).manhattan_distance(access_point),
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(answer(1), Answer{steps: 0});
        assert_eq!(answer(12), Answer{steps: 3});
        assert_eq!(answer(23), Answer{steps: 2});
        assert_eq!(answer(1024), Answer{steps: 31});
    }

    #[test]
    fn manhattan_distance_symmetrical() {
        let centre = Point(0, 0);
        assert_eq!(centre.manhattan_distance(Point(1, 1)), centre.manhattan_distance(Point(-1, -1)));
        assert_eq!(centre.manhattan_distance(Point(1, 2)), centre.manhattan_distance(Point(-1, -2)));
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
        assert_eq!(Point::from(23), Point(0, -3));
        assert_eq!(Point::from(25), Point(2, -2));
        assert_eq!(Point::from(26), Point(3, -2));
        assert_eq!(Point::from(45), Point(-1, -3));
    }

}
