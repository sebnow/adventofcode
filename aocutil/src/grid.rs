use std::collections::HashMap;
use std::iter;
use std::iter::FromIterator;

use euclid::Box2D;

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
        v.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &cell)| (Point::new(x as i64, 0 - y as i64), cell))
            })
            .collect()
    }

    pub fn insert(&mut self, p: Point, v: T) {
        self.coords.insert(p, v);
        self.x_bounds = (self.x_bounds.0.min(p.x), self.x_bounds.1.max(p.x));
        self.y_bounds = (self.y_bounds.0.min(p.y), self.y_bounds.1.max(p.y));
    }

    pub fn remove(&mut self, p: &Point) -> Option<T> {
        let c = self.coords.remove(p);

        if c.is_some() {
            self.reset_bounds()
        }

        c
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.coords.get(p)
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut T> {
        self.coords.get_mut(p)
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

    pub fn len(&self) -> usize {
        self.coords.len()
    }

    pub fn bounds(&self) -> Box2D<i64, euclid::UnknownUnit> {
        Box2D::new(
            Point::new(self.x_bounds.0, self.y_bounds.0),
            Point::new(self.x_bounds.1, self.y_bounds.1),
        )
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
    pub fn surrounding(&self, p: &Point, mask: u8) -> Surrounding<T> {
        Surrounding::new(self, p, mask)
    }

    fn reset_bounds(&mut self) {
        let mut x_bounds = (0, 0);
        let mut y_bounds = (0, 0);

        for p in self.coords.keys() {
            x_bounds = (x_bounds.0.min(p.x), x_bounds.1.max(p.x));
            y_bounds = (y_bounds.0.min(p.y), y_bounds.1.max(p.y));
        }

        self.x_bounds = x_bounds;
        self.y_bounds = y_bounds;
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
                    write!(f, "{}", T::default())?;
                }
            }

            if y != self.y_bounds.0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> FromIterator<(Point, T)> for Grid<T>
where
    T: PartialEq + std::marker::Copy,
{
    fn from_iter<I: IntoIterator<Item = (Point, T)>>(iter: I) -> Self {
        let mut g = Grid::new();
        g.extend(iter);
        g
    }
}

impl<T> std::str::FromStr for Grid<T>
where
    T: From<char> + PartialEq + std::marker::Copy,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .enumerate()
            .flat_map(move |(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| (Point::new(x as i64, 0 - y as i64), c.into()))
            })
            .collect())
    }
}

impl<T> Extend<(Point, T)> for Grid<T>
where
    T: PartialEq + std::marker::Copy,
{
    fn extend<I: IntoIterator<Item = (Point, T)>>(&mut self, iter: I) {
        for (p, c) in iter {
            self.insert(p, c);
        }
    }
}

#[must_use]
pub struct Surrounding<'a, T> {
    grid: &'a Grid<T>,
    points: [Option<Point>; 8],
    index: usize,
}

impl<'a, T> Surrounding<'a, T> {
    pub fn new(grid: &'a Grid<T>, point: &Point, mask: u8) -> Self {
        let all = [
            Point::new(point.x - 1, point.y + 1),
            Point::new(point.x, point.y + 1),
            Point::new(point.x + 1, point.y + 1),
            Point::new(point.x - 1, point.y),
            Point::new(point.x + 1, point.y),
            Point::new(point.x - 1, point.y - 1),
            Point::new(point.x, point.y - 1),
            Point::new(point.x + 1, point.y - 1),
        ];

        let mut points = [None; 8];

        let mut idx = 0;
        for (bit, &p) in all.iter().enumerate() {
            let cell = 1 << (7 - bit);
            if mask & cell == cell {
                points[idx] = Some(p);
                idx += 1;
            }
        }

        Surrounding {
            grid,
            index: 0,
            points,
        }
    }
}

impl<'a, T> Iterator for Surrounding<'a, T>
where
    T: PartialEq + Copy,
{
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let &Some(p) = self.points.get(self.index)? {
            self.index += 1;

            if let Some(c) = self.grid.get(&p) {
                return Some((p, c));
            }
        }

        None
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
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![
                (Point::new(0, 0), &'a'),
                (Point::new(2, 0), &'c'),
                (Point::new(0, -2), &'g'),
                (Point::new(2, -2), &'i'),
            ],
            g.surrounding(&Point::new(1, -1), !MASK_CROSSHAIR)
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![(Point::new(1, 0), &'b'), (Point::new(0, -1), &'d'),],
            g.surrounding(&Point::new(0, 0), MASK_CROSSHAIR)
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![
                (Point::new(1, 0), &'b'),
                (Point::new(0, -1), &'d'),
                (Point::new(1, -1), &'e'),
            ],
            g.surrounding(&Point::new(0, 0), MASK_ALL)
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![(Point::new(2, -1), &'f'), (Point::new(1, -2), &'h'),],
            g.surrounding(&Point::new(2, -2), MASK_CROSSHAIR)
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![
                (Point::new(1, -1), &'e'),
                (Point::new(2, -1), &'f'),
                (Point::new(1, -2), &'h'),
            ],
            g.surrounding(&Point::new(2, -2), MASK_ALL)
                .collect::<Vec<_>>()
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
                .collect::<Vec<_>>()
        );

        assert_eq!(
            Vec::<(Point, &char)>::new(),
            g.surrounding(&Point::new(1, -1), !MASK_ALL)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn from_tuple_iter() {
        let g: Grid<char> = [(Point::new(0, 0), 'a'), (Point::new(2, 1), 'c')]
            .iter()
            .copied()
            .collect();

        assert_eq!(g.get(&Point::new(0, 0)).unwrap(), &'a');
        assert_eq!(g.get(&Point::new(2, 1)).unwrap(), &'c');
    }

    #[test]
    fn extend_tuple() {
        let mut g: Grid<char> = [(Point::new(0, 0), 'a')].iter().copied().collect();

        g.extend([(Point::new(2, 1), 'c')]);

        assert_eq!(g.get(&Point::new(0, 0)).unwrap(), &'a');
        assert_eq!(g.get(&Point::new(2, 1)).unwrap(), &'c');
    }
}
