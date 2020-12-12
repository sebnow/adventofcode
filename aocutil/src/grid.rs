use euclid;
use std::collections::HashMap;
use std::iter;

pub trait Collision {
    fn is_collidable(&self) -> bool;
}

pub type Point = euclid::Point2D<i64, euclid::UnknownUnit>;
pub type Vector = euclid::Vector2D<i64, euclid::UnknownUnit>;

pub const MASK_CROSSHAIR: u8 = 0b01011010;
pub const MASK_ALL: u8 = 0b11111111;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Grid<T> {
    coords: HashMap<Point, T>,
    x_bounds: (i64, i64),
    y_bounds: (i64, i64),
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid {
            coords: HashMap::default(),
            x_bounds: (0, 0),
            y_bounds: (0, 0),
        }
    }
}

impl<T> Grid<T>
where
    T: Copy + PartialEq,
{
    // TODO: Make this generic over any nested iterable
    pub fn from_vec2d(v: Vec<Vec<T>>) -> Self {
        let mut g = Grid {
            coords: HashMap::default(),
            x_bounds: (0, 0),
            y_bounds: (0, 0),
        };

        for (y, row) in v.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                g.insert(Point::new(x as i64, 0 - y as i64), *cell);
            }
        }

        g
    }

    pub fn insert(&mut self, p: Point, v: T) {
        self.coords.insert(p, v);
        self.x_bounds = (self.x_bounds.0.min(p.x), self.x_bounds.1.max(p.x));
        self.y_bounds = (self.y_bounds.0.min(p.y), self.y_bounds.1.max(p.y));
    }

    pub fn remove(&mut self, p: &Point) -> Option<T> {
        self.coords.remove(p)
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.coords.get(p)
    }

    pub fn iter(&self) -> impl iter::Iterator<Item = (&Point, &T)> {
        self.coords.iter()
    }

    pub fn rows(&self) -> usize {
        (self.y_bounds.1 - self.y_bounds.0) as usize
    }

    pub fn cols(&self) -> usize {
        (self.x_bounds.1 - self.x_bounds.0) as usize
    }

    /// Return cells surrounding `p` according to `mask`. The `mask` bit positions map to cells in
    /// row-major order starting with the most significant bit, omitting the middle point `p`.
    ///
    /// +---+---+---+
    /// | 7 | 6 | 5 |
    /// +---+---+---+
    /// | 4 | _ | 3 |
    /// +---+---+---+
    /// | 2 | 1 | 0 |
    /// +---+---+---+
    pub fn surrounding(&self, p: &Point, mask: u8) -> Vec<(Point, &T)> {
        [
            Point::new(p.x - 1, p.y + 1),
            Point::new(p.x, p.y + 1),
            Point::new(p.x + 1, p.y + 1),
            Point::new(p.x - 1, p.y),
            Point::new(p.x + 1, p.y),
            Point::new(p.x - 1, p.y - 1),
            Point::new(p.x, p.y - 1),
            Point::new(p.x + 1, p.y - 1),
        ]
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            let cell = 1 << (7 - idx);
            if mask & cell == cell {
                self.coords.get(s).map(|v| (s.clone(), v))
            } else {
                None
            }
        })
        .collect()
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display + std::default::Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (self.y_bounds.0..=self.y_bounds.1).rev() {
            for x in self.x_bounds.0..=self.x_bounds.1 {
                if let Some(x) = self.coords.get(&Point::new(x as i64, y as i64)) {
                    write!(f, "{}", x)?;
                } else {
                    write!(f, " ")?;
                }
            }

            if y != self.y_bounds.0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Default, PartialEq)]
    struct Collidable<T>(T, bool);

    impl<T> Collision for Collidable<T> {
        fn is_collidable(&self) -> bool {
            self.1
        }
    }

    #[test]
    fn display_coords() {
        let mut g = Grid::default();
        g.insert(Point::new(0, 0), "a");
        g.insert(Point::new(2, 0), "b");
        g.insert(Point::new(1, -1), "c");
        g.insert(Point::new(0, -2), "d");
        g.insert(Point::new(2, -2), "e");

        assert_eq!(
            r#"a b
 c 
d e"#,
            format!("{}", g)
        );
    }

    #[test]
    fn from_vec2d() {
        let mut g = Grid::default();
        g.insert(Point::new(0, 0), 'a');
        g.insert(Point::new(1, 0), ' ');
        g.insert(Point::new(2, 0), 'b');
        g.insert(Point::new(0, -1), ' ');
        g.insert(Point::new(1, -1), 'c');
        g.insert(Point::new(2, -1), ' ');
        g.insert(Point::new(0, -2), 'd');
        g.insert(Point::new(1, -2), ' ');
        g.insert(Point::new(2, -2), 'e');

        assert_eq!(
            g,
            Grid::from_vec2d(vec![
                vec!['a', ' ', 'b'],
                vec![' ', 'c', ' '],
                vec!['d', ' ', 'e'],
            ])
        );
    }

    #[test]
    fn surrounding() {
        let g = Grid::from_vec2d(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        assert_eq!(
            vec![
                (Point::new(1, 0), &'b'),
                (Point::new(0, -1), &'d'),
                (Point::new(2, -1), &'f'),
                (Point::new(1, -2), &'h')
            ],
            g.surrounding(&Point::new(1, -1), MASK_CROSSHAIR)
        );

        assert_eq!(
            vec![
                (Point::new(0, 0), &'a'),
                (Point::new(2, 0), &'c'),
                (Point::new(0, -2), &'g'),
                (Point::new(2, -2), &'i'),
            ],
            g.surrounding(&Point::new(1, -1), !MASK_CROSSHAIR)
        );

        assert_eq!(
            vec![
                (Point::new(0, 0), &'a'),
                (Point::new(1, 0), &'b'),
                (Point::new(2, 0), &'c'),
                (Point::new(0, -1), &'d'),
                (Point::new(2, -1), &'f'),
                (Point::new(0, -2), &'g'),
                (Point::new(1, -2), &'h'),
                (Point::new(2, -2), &'i')
            ],
            g.surrounding(&Point::new(1, -1), MASK_ALL)
        );

        assert_eq!(
            Vec::<(Point, &char)>::new(),
            g.surrounding(&Point::new(1, -1), !MASK_ALL)
        );
    }
}
