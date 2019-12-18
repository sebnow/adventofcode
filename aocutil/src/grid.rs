use crate::Point;
use anyhow::Result;
use std::collections::HashMap;

use std::io;
use std::io::Write;

use termion::cursor::HideCursor;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

#[derive(Debug)]
pub struct Grid<T> {
    top_left: Point<i64>,
    bottom_right: Point<i64>,
    pub points: HashMap<Point<i64>, T>,
}

impl<T> Grid<T>
where
    T: std::fmt::Display,
{
    pub fn new() -> Self {
        Grid::default()
    }

    pub fn add(&mut self, p: Point<i64>, v: T) {
        self.top_left = Point::new(self.top_left.x.min(p.x), self.top_left.y.max(p.y));
        self.bottom_right = Point::new(self.bottom_right.x.max(p.x), self.bottom_right.y.min(p.y));
        self.points.insert(p, v);
    }

    pub fn remove(&mut self, p: &Point<i64>) -> Option<T> {
        self.points.remove(p)
    }

    pub fn at(&self, p: &Point<i64>) -> Option<&T> {
        self.points.get(p)
    }

    pub fn adjacent<'a>(&'a self, p: &Point<i64>) -> Vec<(Point<i64>, &'a T)> {
        [
            Point::new(p.x - 1, p.y),
            Point::new(p.x + 1, p.y),
            Point::new(p.x, p.y - 1),
            Point::new(p.x, p.y + 1),
        ]
        .iter()
        .filter_map(|adj| self.at(adj).map(|t| (*adj, t)))
        .collect()
    }

    pub fn render(&self) -> Result<()> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = HideCursor::from(stdout);
        let mut stdout = AlternateScreen::from(stdout);

        write!(stdout, "{}", termion::clear::All)?;
        for (p, v) in &self.points {
            let p = Point::new(self.top_left.x + p.x + 1, self.top_left.y - p.y + 1);
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(p.x as u16, p.y as u16),
                v
            )?;
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point<i64>, &T)> {
        self.points.iter()
    }

    pub fn flip_y(&mut self) {
        let keys: Vec<Point<_>> = self.points.keys().copied().collect();
        let mut top_left = keys[0];
        let mut bottom_right = keys[0];

        for p in keys {
            let v = self.points.remove(&p).unwrap();
            self.points.insert(Point::new(p.x, 0 - p.y), v);

            top_left.x = top_left.x.min(p.x);
            top_left.y = top_left.y.max(p.y);
            bottom_right.x = bottom_right.x.max(p.x);
            bottom_right.y = bottom_right.y.min(p.y);
        }

        self.top_left = top_left;
        self.bottom_right = bottom_right;
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Grid {
            top_left: Point::default(),
            bottom_right: Point::default(),
            points: Default::default(),
        }
    }
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.top_left.y - self.bottom_right.y;
        let width = self.bottom_right.x - self.top_left.x;

        println!("{} x {}", height, width);
        for y in (0..=height).rev() {
            for x in 0..=width {
                let p = &Point::new(self.top_left.x + x, y - self.top_left.y);
                match self.points.get(&p) {
                    Some(v) => write!(f, "{}", v),
                    None => write!(f, " "),
                }?;
            }
            if y != 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> From<&HashMap<Point<i64>, T>> for Grid<T>
where
    T: std::fmt::Display + Copy,
{
    fn from(m: &HashMap<Point<i64>, T>) -> Grid<T> {
        let mut grid = Grid::default();
        for (p, v) in m {
            grid.add(*p, *v);
        }

        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_grid() {
        let mut g = Grid::new();

        let points = &[
            Point::new(-2, 1),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(-1, -1),
            Point::new(2, -1),
        ];
        for &p in points {
            g.add(p, '#');
        }

        assert_eq!(format!("{}", g), "# # #\n # # \n #  #");

        let mut g = Grid::new();

        let points = &[Point::new(0, 2), Point::new(1, 1), Point::new(1, 0)];
        for &p in points {
            g.add(p, '#');
        }

        assert_eq!(format!("{}", g), "# \n #\n #");
    }
}
