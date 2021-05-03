use super::iter::*;
use super::pattern::*;
use super::step::*;
use std::mem;


/// Position
/// A position in the Grid.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (usize, usize) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

/// Grid
/// 2D Grid, Position (0,0) is at the top left corner 
#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    pub(crate) items: Vec<T>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl<T: Clone> Grid<T> {
    /// Creates a new Grid with `default_value` as every value. 
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let grid: Grid<u8> = Grid::new(2, 2, 10);
    /// assert_eq!(grid.get((0, 0)), Some(&10));
    /// assert_eq!(grid.get((1, 0)), Some(&10));
    /// assert_eq!(grid.get((0, 1)), Some(&10));
    /// assert_eq!(grid.get((1, 1)), Some(&10));
    /// ```
    /// # Panics
    /// * if width or height are zero
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        if width == 0 || height == 0 {
            panic!("width and height can not be zero");
        }
        Self {
            width,
            height,
            items: vec![default_value; width * height],
        }
    }
}

impl<T: Default> Grid<T> {
    /// Returns the item at `pos` and leaves `T::Default()` in it's place,
    /// or `None` if `pos` is out of bounds.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid: Grid<usize> = Grid::new(2, 2, 10); 
    /// let old = grid.replace_default((1, 1));
    /// assert_eq!(old, Some(10));
    /// assert_eq!(grid.get((1, 1)), Some(&0));
    /// ```
    pub fn replace_default<P: Into<Position>>(&mut self, pos: P) -> Option<T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            let old = mem::replace(&mut self.items[idx], T::default());
            return Some(old);
        }
        None
    }

    /// Moves the item at `pos` to position `to`, overrides item at `to` in the process,
    /// and leaves the `T::Default()` in `pos`.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid: Grid<usize> = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// grid.move_to((0, 0), (1, 1));
    /// assert_eq!(grid.get((0, 0)), Some(&0));
    /// assert_eq!(grid.get((1, 1)), Some(&1));
    /// ```
    /// # Panics
    /// * if positions `pos` or `to` are out of bounds
    pub fn move_to<P: Into<Position>>(&mut self, pos: P, to: P) {
        let pos = pos.into();
        let to = to.into();

        if !self.is_bounds(pos) && !self.is_bounds(to) {
            panic!("Out of bounds");
        }

        let idx_to = self.translate(to);
        self.items[idx_to] = self.replace_default(pos).unwrap();
    }
}

impl<T> Grid<T> {
    /// Constructs a new Grid with items in Vector `v`
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// ```
    /// # Panics
    /// * if width or height is zero
    /// * if `v` length is not equal width times height
    pub fn from(v: Vec<T>, width: usize, height: usize) -> Self {
        if v.len() != (width * height) {
            panic!("v length does not equal width * height");
        }
        if width == 0 || height == 0 {
            panic!("width and height can not be zero");
        }
        Self {
            items: v,
            width,
            height,
        }
    }

    #[inline]
    fn translate<P: Into<Position>>(&self, pos: P) -> usize {
        let pos = pos.into();
        pos.y * self.width + pos.x
    }

    /// Checks if position `pos` is in bounds of the grid.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let grid = Grid::new(2, 2, 0usize);
    /// assert_eq!(grid.is_bounds((1, 1)), true);
    /// assert_eq!(grid.is_bounds((2, 2)), false);
    /// ```
    #[inline]
    pub fn is_bounds<P: Into<Position>>(&self, pos: P) -> bool {
        let pos = pos.into();
        pos.x < self.width && pos.y < self.height
    }

    /// Returns the width and height of the grid.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// assert_eq!(grid.size(), (2, 2));
    /// ```
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Returns the full length of the grid
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// assert_eq!(grid.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns a reference to an element at position `pos` 
    /// or `None`, if `pos` is out of bounds.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// assert_eq!(grid.get((1, 0)), Some(&2));
    /// ```
    pub fn get<P: Into<Position>>(&self, pos: P) -> Option<&T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            return Some(&self.items[idx]);
        }
        None
    }

    /// Returns a mutable reference to an element at position `pos` 
    /// or None, if `pos` is out of bounds.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// assert_eq!(grid.get_mut((0, 1)), Some(&mut 3));
    /// ```
    pub fn get_mut<P: Into<Position>>(&mut self, pos: P) -> Option<&mut T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            return Some(&mut self.items[idx]);
        }
        None
    }

    /// Returns a reference to an element at position `pos` without bound checks. 
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// assert_eq!(grid.get_unchecked((1, 1)), &4);
    /// ```
    /// # Safety
    /// Does not do any bound checks.  
    /// `pos` does not have to be in bounds as long pos.x*pos.y < grid.len()  
    /// for example on a grid size 3,3: `get_unchecked(8,0)` will return the last element
    /// # Panics
    /// * if pos.x times pos.y  is greater than grid length.
    pub fn get_unchecked<P: Into<Position>>(&self, pos: P) -> &T {
        let idx = self.translate(pos);
        &self.items[idx]
    }

    /// Returns a reference to an element at position `pos` without bound checks.  
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// assert_eq!(grid.get_mut_unchecked((1, 1)), &mut 4);
    /// ```
    /// # Safety
    /// Does not do any bound checks.  
    /// `pos` does not have to be in bounds as long pos.x*pos.y < grid.len()  
    /// for example on a grid size 3,3: `get_unchecked(8,0)` will return the last element
    /// # Panics
    /// * if pos.x times pos.y  is greater than grid length.
    pub fn get_mut_unchecked<P: Into<Position>>(&mut self, pos: P) -> &mut T {
        let idx = self.translate(pos);
        &mut self.items[idx]
    }

    /// Sets the value at position `pos`.
    /// Returns None if `pos` is out of bounds,
    /// or () otherwise.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// grid.set((0, 0), 10);
    /// assert_eq!(grid.get((0, 0)), Some(&10));
    /// ```
    pub fn set<P: Into<Position>>(&mut self, pos: P, value: T) -> Option<()> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            self.items[idx] = value;
        }
        None
    }

    /// Sets the value at position `pos`, without bound checks.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// grid.set_unchecked((0, 0), 10);
    /// assert_eq!(grid.get((0, 0)), Some(&10));
    /// ```
    /// # Safety
    /// Does not do any bound checks.  
    /// `pos` does not have to be in bounds as long pos.x*pos.y < grid.len()  
    /// for example on a grid size 3,3: `get_unchecked(8,0)` will return the last element
    /// # Panics
    /// * if pos.x times pos.y  is greater than grid length.
    pub fn set_unchecked<P: Into<Position>>(&mut self, pos: P, value: T) {
        let idx = self.translate(pos);
        self.items[idx] = value;
    }

    /// Replace the value at position `pos` and returns the old value,
    /// or `None` if `pos` is out of bounds.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let old = grid.replace((0, 0), 10);
    /// assert_eq!(old, Some(1));
    /// assert_eq!(grid.get((0, 0)), Some(&10));
    /// ```
    pub fn replace<P: Into<Position>>(&mut self, pos: P, value: T) -> Option<T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            let old = mem::replace(&mut self.items[idx], value);
            return Some(old);
        }
        None
    }

    /// Swap the values of positions `pos_a` and `pos_b`.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// grid.swap((0, 0), (1, 0));
    /// assert_eq!(grid.get((0, 0)), Some(&2));
    /// assert_eq!(grid.get((1, 0)), Some(&1));
    /// ```
    /// # Panics
    /// * if position `pos` is out of bounds.
    pub fn swap<P: Into<Position>>(&mut self, pos_a: P, pos_b: P) {
        let pos_a = pos_a.into();
        let pos_b = pos_b.into();
        if !self.is_bounds(pos_a) && !self.is_bounds(pos_b) {
            panic!("Out of bounds");
        }

        let idx_a = self.translate(pos_a);
        let idx_b = self.translate(pos_b);
        self.items.swap(idx_a, idx_b);
    }

    /// Move the value of position `pos` to position `to` and leaves `value` in it's place.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// grid.move_and_leave((0, 0), (1, 1), 42);
    /// assert_eq!(grid.get((0, 0)), Some(&42));
    /// assert_eq!(grid.get((1, 1)), Some(&1));
    /// ```
    /// # Panics
    /// * if position `pos` is out of bounds
    pub fn move_and_leave<P: Into<Position>>(&mut self, pos: P, to: P, value: T) {
        let pos = pos.into();
        let to = to.into();
        if !self.is_bounds(pos) && !self.is_bounds(to) {
            panic!("Out of bound");
        }

        let idx_to = self.translate(to);
        self.items[idx_to] = self.replace(pos, value).unwrap();
    }

    /// Creates an iterator which yields all positions of grid.  
    /// # Example
    /// ```
    /// # use gridit::{Grid, Position};
    /// let mut grid = Grid::new(2, 2, 0);
    /// let mut positions = grid.positions();
    /// assert_eq!(positions.next(), Some(Position::new(0, 0)));
    /// assert_eq!(positions.next(), Some(Position::new(1, 0)));
    /// assert_eq!(positions.next(), Some(Position::new(0, 1)));
    /// assert_eq!(positions.next(), Some(Position::new(1, 1)));
    /// assert_eq!(positions.next(), None);
    /// ```
    pub fn positions(&self) -> PositionsIter {
        PositionsIter {
            len: self.items.len(),
            width: self.width,
            idx: 0,
        }
    }

    /// Creates an iterator which yields references of every element in grid.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let mut iter = grid.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid_iter: self.items.iter(),
            width: self.width,
        }
    }

    /// Creates an iterator which yields mutable references of every element in grid.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let mut iter = grid.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 4));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut<'a>(&'a mut self) -> GridIterMut<'a, T> {
        GridIterMut {
            grid_iter: self.items.iter_mut(),
            width: self.width,
        }
    }

    /// Creates an iterator which yields references of every element in row `y`.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let mut row = grid.row(0);
    /// assert_eq!(row.next(), Some(&1));
    /// assert_eq!(row.next(), Some(&2));
    /// assert_eq!(row.next(), None);
    /// ```
    /// # Panics
    /// * if the row is out of bounds.
    pub fn row<'a>(&'a self, y: usize) -> RowIter<'a, T> {
        assert!(self.is_bounds((0, y)));
        let start_idx = y * self.width;
        let end_idx = start_idx + self.width;

        RowIter {
            row_iter: self.items[start_idx..end_idx].iter(),
            idx: y,
        }
    }

    /// Creates an iterator which yields mutable references of every element in row `y`.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let mut row = grid.row_mut(1);
    /// assert_eq!(row.next(), Some(&mut 3));
    /// assert_eq!(row.next(), Some(&mut 4));
    /// assert_eq!(row.next(), None);
    /// ```
    /// # Panics
    /// * if the row is out of bounds.
    pub fn row_mut<'a>(&'a mut self, y: usize) -> RowIterMut<'a, T> {
        assert!(self.is_bounds((0, y)));
        let start_idx = y * self.width;
        let end_idx = start_idx + self.width;

        RowIterMut {
            row_iter: self.items[start_idx..end_idx].iter_mut(),
            idx: y,
        }
    }

    /// Creates an iterator which yields references of every element in column `x`.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let mut column = grid.column(0);
    /// assert_eq!(column.next(), Some(&1));
    /// assert_eq!(column.next(), Some(&3));
    /// assert_eq!(column.next(), None);
    /// ```
    /// # Panics
    /// * if the column is out of bounds.
    pub fn column<'a>(&'a self, x: usize) -> ColumnIter<'a, T> {
        assert!(self.is_bounds((x, 0)));
        ColumnIter {
            row_idx: 0,
            col_idx: x,
            grid: &self,
        }
    }

    /// Creates an iterator which yields mutable references of every element in column `x`.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let mut column = grid.column_mut(1);
    /// assert_eq!(column.next(), Some(&mut 2));
    /// assert_eq!(column.next(), Some(&mut 4));
    /// assert_eq!(column.next(), None);
    /// ```
    /// # Panics
    /// * if the column is out of bounds.
    pub fn column_mut<'a>(&'a mut self, x: usize) -> ColumnIterMut<'a, T> {
        assert!(self.is_bounds((x, 0)));
        let width = self.width;
        let iter = self.iter_mut().skip(x).step_by(width);
        ColumnIterMut { iter, col_idx: x }
    }

    // Returns every valid neighbor position of x,y
    fn get_neighbor_positions<P: Into<Position>>(&self, pos: P) -> Vec<Position> {
        let Position { x, y } = pos.into();
        let neighbor_position: [(N, N); 8] = [
            (N::N(1), N::N(1)),
            (N::P(0), N::N(1)),
            (N::P(1), N::N(1)),
            (N::N(1), N::P(0)),
            (N::P(1), N::P(0)),
            (N::N(1), N::P(1)),
            (N::P(0), N::P(1)),
            (N::P(1), N::P(1)),
        ];

        let valid_positions: Vec<Position> = neighbor_position
            .iter()
            .filter_map(|(nx, ny)| {
                let x = nx.checked_add_sub(x)?;
                let y = ny.checked_add_sub(y)?;

                if self.get((x, y)).is_some() {
                    return Some((x, y).into());
                }
                None
            })
            .collect();

        valid_positions
    }

    /// Creates an iterator which yields references of every neighbor element of position `pos`.
    /// # Example
    /// ```
    /// # use gridit::Grid;
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2);
    /// let mut neighbors = grid.neighbors((0, 1));
    /// assert_eq!(neighbors.next(), Some(&1));
    /// assert_eq!(neighbors.next(), Some(&2));
    /// assert_eq!(neighbors.next(), Some(&4));
    /// assert_eq!(neighbors.next(), None);
    /// ```
    /// # Panics
    /// * if x or y is out of bounds.
    pub fn neighbors<'a, P: Into<Position>>(&'a self, pos: P) -> NeighborIter<'a, T> {
        let pos = pos.into();
        assert!(self.is_bounds(pos));
        NeighborIter {
            positions: self.get_neighbor_positions(pos),
            grid: &self,
            idx: 0,
        }
    }

    /// Creates an iterator which yields references of every element of pattern starting at position `pos`.  
    /// See [Pattern],[DirectionPattern] and [SequencePattern] for more details.
    /// # Example
    /// ```
    /// # use gridit::{Grid, SequencePattern};
    /// let mut grid = Grid::from(vec![1, 2, 3, 4], 2, 2); 
    /// let pattern = SequencePattern::new(vec![(1,0), (-1, 0), (1, 0)]);
    /// let mut iter = grid.pattern((0, 0), pattern);
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    pub fn pattern<'a, P, Pat>(&'a self, pos: P, pattern: Pat) -> PatternIter<'a, T>
    where
        P: Into<Position>,
        Pat: Pattern + 'static,
    {
        let pos = pos.into();
        PatternIter {
            grid: &self,
            prev_position: pos,
            pattern: Box::new(pattern),
            repeat_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grid() {
        let grid = Grid::new(3, 5, 0u8);
        assert_eq!(
            grid,
            Grid {
                width: 3,
                height: 5,
                items: vec![0u8; 3 * 5]
            }
        );
    }

    #[test]
    fn get_cell_in_grid() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let cell = grid.get((1, 1));
        assert_eq!(cell, Some(&2));
    }

    #[test]
    fn get_mut_cell_in_grid() {
        let mut grid = Grid {
            width: 3,
            height: 3,
            items: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let mut_cell = grid.get_mut((1, 1));
        assert_eq!(mut_cell, Some(&mut 2));
    }

    #[test]
    fn get_unchecked_cell_in_grid() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let cell = grid.get_unchecked((1, 1));
        assert_eq!(cell, &2);
    }

    #[test]
    #[should_panic]
    fn get_unchecked_panic_cell_in_grid() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let _cell = grid.get_unchecked((3, 2));
    }

    #[test]
    fn set_cell_in_grid() {
        let mut grid = Grid::new(3, 5, 1u8);
        grid.set((2, 2), 2u8);
        let cell = grid.get((2, 2));
        assert_eq!(cell, Some(&2));
    }

    #[test]
    fn set_unchecked_cell_in_grid() {
        let mut grid = Grid::new(3, 5, 1u8);
        grid.set_unchecked((2, 2), 2u8);
        let cell = grid.get((2, 2));
        assert_eq!(cell, Some(&2));
    }

    #[test]
    #[should_panic]
    fn set_unchecked_panic_cell_in_grid() {
        let mut grid = Grid::new(3, 3, 1u8);
        grid.set_unchecked((2, 3), 2u8);
    }

    #[test]
    fn replace_cell_in_grid() {
        let mut grid = Grid::new(2, 2, 1u8);
        let value = grid.replace((1, 1), 2u8);
        assert_eq!(value, Some(1));
        assert_eq!(grid.items, vec![1, 1, 1, 2]);
    }

    #[test]
    fn replace_default() {
        let mut grid = Grid::new(2, 2, 1u8);
        grid.replace_default((1, 1));
        assert_eq!(grid.get((1, 1)), Some(&0));
    }

    #[test]
    fn swap() {
        let mut grid = Grid {
            items: (0..6).collect(),
            width: 2,
            height: 3,
        };

        grid.swap((1, 2), (0, 1));
        assert_eq!(grid.get((1, 2)), Some(&2));
        assert_eq!(grid.get((0, 1)), Some(&5));
    }

    #[test]
    fn move_to() {
        let mut grid = Grid {
            items: (0..4).collect(),
            width: 2,
            height: 2,
        };

        grid.move_to((1, 1), (0, 1));
        assert_eq!(grid.get((1, 1)), Some(&0));
        assert_eq!(grid.get((0, 1)), Some(&3));
    }

    #[test]
    fn move_and_leave() {
        let mut grid = Grid {
            items: (0..4).collect(),
            width: 2,
            height: 2,
        };

        grid.move_and_leave((1, 0), (0, 0), 10);
        assert_eq!(grid.get((1, 0)), Some(&10));
        assert_eq!(grid.get((0, 0)), Some(&1));
    }
}
