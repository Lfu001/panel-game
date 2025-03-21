use serde::{Deserialize, Serialize};

use crate::colors::{Color, ColorMap, to_rgb};

/// A rectangle.
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    /// Creates a new [`Rectangle`].
    #[cfg(test)]
    pub fn new(width: usize, height: usize) -> Rectangle {
        Rectangle { width, height }
    }

    /// Returns the width of this [`Rectangle`].
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this [`Rectangle`].
    pub fn height(&self) -> usize {
        self.height
    }

    /// Swaps the width and height of this [`Rectangle`].
    pub fn transpose(&mut self) {
        std::mem::swap(&mut self.width, &mut self.height);
    }

    /// Calculates the area of this [`Rectangle`].
    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

/// A two-dimensional position.
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    /// Creates a new [`Position`].
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    /// Returns the x of this [`Position`].
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns the y of this [`Position`].
    pub fn y(&self) -> usize {
        self.y
    }
}

/// A two-dimensional grid of values.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    /// Creates a new [`Grid<T>`].
    pub fn new(rows: usize, cols: usize, value: T) -> Grid<T> {
        Grid {
            rows,
            cols,
            data: vec![vec![value; cols]; rows],
        }
    }

    /// Returns the rows of this [`Grid<T>`].
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the cols of this [`Grid<T>`].
    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl Grid<f64> {
    /// Convert the elements of this [`Grid<f64>`] to (value, color) pairs.
    pub fn to_value_color_pairs(&self, cmap: &ColorMap) -> Grid<(f64, Color)> {
        let mut res = Grid::new(self.rows, self.cols, (0.0, Color(0, 0, 0)));
        for j in 0..self.rows {
            for i in 0..self.cols {
                let pos = Position::new(i, j);
                let elem = self[&pos];
                let color = to_rgb(elem, cmap);
                res[&pos] = (elem, color);
            }
        }
        res
    }
}

impl std::ops::Div<f64> for Grid<f64> {
    type Output = Grid<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let mut new_data = self.data.clone();
        for row in &mut new_data {
            for elem in row {
                *elem /= rhs;
            }
        }
        Grid {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }
}

impl std::ops::DivAssign<f64> for Grid<f64> {
    fn div_assign(&mut self, rhs: f64) {
        for row in &mut self.data {
            for elem in row {
                *elem /= rhs;
            }
        }
    }
}

macro_rules! impl_index {
    ($t:ty) => {
        impl std::ops::Index<&Position> for Grid<$t> {
            type Output = $t;

            fn index(&self, index: &Position) -> &Self::Output {
                &self.data[index.y()][index.x()]
            }
        }

        impl std::ops::IndexMut<&Position> for Grid<$t> {
            fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
                &mut self.data[index.y()][index.x()]
            }
        }
    };
}

impl_index!(bool);
impl_index!(usize);
impl_index!(f64);
impl_index!((f64, Color));

impl<T: PartialEq> Grid<T> {
    /// Returns `true` if all elements in the specified rectangular area are the same as the given `value`.
    pub fn all(&self, pos: &Position, rect: &Rectangle, value: &T) -> bool {
        if pos.x() + rect.width() > self.cols || pos.y() + rect.height() > self.rows {
            return false;
        }
        for y in pos.y()..pos.y() + rect.height() {
            for x in pos.x()..pos.x() + rect.width() {
                if &self.data[y][x] != value {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_new() {
        let rect = Rectangle::new(3, 4);
        assert_eq!(rect.width(), 3);
        assert_eq!(rect.height(), 4);
    }

    #[test]
    fn test_position_new() {
        let pos = Position::new(5, 6);
        assert_eq!(pos.x(), 5);
        assert_eq!(pos.y(), 6);
    }

    #[test]
    fn test_grid_new() {
        let grid: Grid<f64> = Grid::new(2, 3, 1.0);
        assert_eq!(grid.rows, 2);
        assert_eq!(grid.cols, 3);
        for row in grid.data.iter() {
            for &value in row.iter() {
                assert_eq!(value, 1.0);
            }
        }
    }

    #[test]
    fn test_grid_all() {
        let grid: Grid<bool> = Grid::new(2, 2, true);
        let rect = Rectangle::new(2, 2);
        let pos = Position::new(0, 0);
        assert!(grid.all(&pos, &rect, &true));
    }

    #[test]
    fn test_grid_all_out_of_bounds() {
        let grid: Grid<bool> = Grid::new(2, 2, true);
        let rect = Rectangle::new(2, 2);
        let pos = Position::new(1, 1);
        assert!(!grid.all(&pos, &rect, &true));
    }

    #[test]
    fn test_grid_to_value_color_pairs() {
        let grid: Grid<f64> = Grid::new(2, 2, 0.5);
        let cmap = ColorMap::Magma;
        let result = grid.to_value_color_pairs(&cmap);
        for row in result.data.iter() {
            for &(value, ref color) in row.iter() {
                assert_eq!(value, 0.5);
                assert_eq!(*color, to_rgb(0.5, &cmap));
            }
        }
    }

    #[test]
    fn test_grid_div() {
        let grid: Grid<f64> = Grid::new(2, 2, 4.0);
        let result = grid / 2.0;
        for row in result.data.iter() {
            for &value in row.iter() {
                assert_eq!(value, 2.0);
            }
        }
    }

    #[test]
    fn test_grid_div_assign() {
        let mut grid: Grid<f64> = Grid::new(2, 2, 4.0);
        grid /= 2.0;
        for row in grid.data.iter() {
            for &value in row.iter() {
                assert_eq!(value, 2.0);
            }
        }
    }
}
