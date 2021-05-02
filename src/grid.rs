use super::iter::*;
use super::pattern::Pattern;
use super::step::N;
use std::mem;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
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

//pub type Position = (usize, usize);

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    pub(crate) cells: Vec<T>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

/// Grid
/// 0,0 is at the top left corner and iterators row wise.
impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        Self {
            width,
            height,
            cells: vec![default_value; width * height],
        }
    }
}

impl<T: Default> Grid<T> {
    /// Sets the element at position x,y to the default value of T  
    /// returns the old value of x,y
    /// or None if x or y is out of bounds
    pub fn replace_default<P: Into<Position>>(&mut self, pos: P) -> Option<T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            let old = mem::replace(&mut self.cells[idx], T::default());
            return Some(old);
        }
        None
    }

    pub fn move_to<P: Into<Position>>(&mut self, pos: P, to: P) {
        let pos = pos.into();
        let to = to.into();

        if !self.is_bounds(pos) && !self.is_bounds(to) {
            panic!("Out of bounds");
        }

        let idx_to = self.translate(to);
        self.cells[idx_to] = self.replace_default(pos).unwrap();
    }
}

impl<T> Grid<T> {
    #[inline]
    fn translate<P: Into<Position>>(&self, pos: P) -> usize {
        let pos = pos.into();
        pos.y * self.width + pos.x
    }

    #[inline]
    fn is_bounds<P: Into<Position>>(&self, pos: P) -> bool {
        let pos = pos.into();
        pos.x < self.width && pos.y < self.height
    }

    /// Returns with and height of the grid
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns a reference to an element at position x,y
    /// or None, if x or y are out of bounds.
    pub fn get<P: Into<Position>>(&self, pos: P) -> Option<&T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            return Some(&self.cells[idx]);
        }
        None
    }

    /// Returns a mutable reference to an element at position x,y
    /// or None, if x or y are out of bounds.
    pub fn get_mut<P: Into<Position>>(&mut self, pos: P) -> Option<&mut T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            return Some(&mut self.cells[idx]);
        }
        None
    }

    /// Returns a reference to an element at position x, y.  
    /// Does not do any bound checks.  
    ///     x or y do not have to be in bounds as long x*y < grid.len()  
    ///     e.g on a grid size 3,3: `get_unchecked(8,0)` will return the last element
    ///
    /// # Panics
    ///
    /// Panics if x*y > grid.len().
    pub fn get_unchecked<P: Into<Position>>(&self, pos: P) -> &T {
        let idx = self.translate(pos);
        &self.cells[idx]
    }

    /// Returns a reference to an element at position x, y.  
    /// Does not do any bound checks.  
    ///     x or y do not have to be in bounds as long x*y < grid.len()  
    ///     e.g on a grid size 3,3: `get_unchecked(8,0)` will return the last element  
    ///
    /// # Panics
    ///
    /// Panics if x*y > grid.len().
    pub fn get_mut_unchecked<P: Into<Position>>(&mut self, pos: P) -> &mut T {
        let idx = self.translate(pos);
        &mut self.cells[idx]
    }

    /// Sets the element at position x, y to `value`.  
    /// Returns None if x or y is out of bounds,
    /// or () otherwise
    pub fn set<P: Into<Position>>(&mut self, pos: P, value: T) -> Option<()> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            self.cells[idx] = value;
        }
        None
    }

    /// Sets the element at position x, y to `value`.  
    /// Does not do any bound checks.  
    ///     x or y do not have to be in bounds as long x*y < grid.len()  
    ///     e.g on a grid size 3,3: `set_unchecked(8,0)` will set the last element to `value`  
    ///
    /// # Panics
    ///
    /// Panics if x*y > grid.len().
    pub fn set_unchecked<P: Into<Position>>(&mut self, pos: P, value: T) {
        let idx = self.translate(pos);
        self.cells[idx] = value;
    }

    /// Sets the element at position x,y to `value`  
    /// returns the old value of x,y
    /// or None if x or y is out of bounds
    pub fn replace<P: Into<Position>>(&mut self, pos: P, value: T) -> Option<T> {
        let pos = pos.into();
        if self.is_bounds(pos) {
            let idx = self.translate(pos);
            let old = mem::replace(&mut self.cells[idx], value);
            return Some(old);
        }
        None
    }

    pub fn swap<P: Into<Position>>(&mut self, pos_a: P, pos_b: P) {
        let pos_a = pos_a.into();
        let pos_b = pos_b.into();
        if !self.is_bounds(pos_a) && !self.is_bounds(pos_b) {
            panic!("Out of bounds");
        }

        let idx_a = self.translate(pos_a);
        let idx_b = self.translate(pos_b);
        self.cells.swap(idx_a, idx_b);
    }

    pub fn move_and_leave<P: Into<Position>>(&mut self, pos: P, to: P, value: T) {
        let pos = pos.into();
        let to = to.into();
        if !self.is_bounds(pos) && !self.is_bounds(to) {
            panic!("Out of bound");
        }

        let idx_to = self.translate(to);
        self.cells[idx_to] = self.replace(pos, value).unwrap();
    }

    /// Creates an iterator over all 2D positions in the Grid.  
    /// Iterator yields the positions as tuple of usize e.g (usize, usize).
    pub fn positions(&self) -> PositionsIter {
        PositionsIter {
            len: self.cells.len(),
            width: self.width,
            idx: 0,
        }
    }

    /// Creates an iterator over every elements in the grid.
    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid_iter: self.cells.iter(),
            width: self.width,
        }
    }

    /// Creates an mutable iterator over every element in the grid.
    pub fn iter_mut<'a>(&'a mut self) -> GridIterMut<'a, T> {
        GridIterMut {
            grid_iter: self.cells.iter_mut(),
            width: self.width,
        }
    }

    /// Creates an iterator over a specific row in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the row is out of bounds.
    pub fn row<'a>(&'a self, y: usize) -> RowIter<'a, T> {
        assert!(self.is_bounds((0, y)));
        let start_idx = y * self.width;
        let end_idx = start_idx + self.width;

        RowIter {
            row_iter: self.cells[start_idx..end_idx].iter(),
            idx: y,
        }
    }

    /// CVectorreates an mutable iterator over a specific row in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the row is out of bounds.
    pub fn row_mut<'a>(&'a mut self, y: usize) -> RowIterMut<'a, T> {
        assert!(self.is_bounds((0, y)));
        let start_idx = y * self.width;
        let end_idx = start_idx + self.width;

        RowIterMut {
            row_iter: self.cells[start_idx..end_idx].iter_mut(),
            idx: y,
        }
    }

    /// Creates an iterator over a specific column in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the column is out of bounds.
    pub fn column<'a>(&'a self, x: usize) -> ColumnIter<'a, T> {
        assert!(self.is_bounds((x, 0)));
        ColumnIter {
            row_idx: 0,
            col_idx: x,
            grid: &self,
        }
    }

    /// Creates an mutable iterator over a specific column in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the column is out of bounds.
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

    /// Returns an iterator over every neighbor from position x,y.
    ///
    /// # Panics
    ///
    /// Panics if x or y is out of bounds.
    pub fn neighbors<'a, P: Into<Position>>(&'a self, pos: P) -> NeighborIter<'a, T> {
        let pos = pos.into();
        assert!(self.is_bounds(pos));
        NeighborIter {
            positions: self.get_neighbor_positions(pos),
            grid: &self,
            idx: 0,
        }
    }

    // pub fn neighbors_mut<'a>(&'a mut self, x: usize, y: usize) -> NeighborIterMut<'a, T> {
    //     NeighborIterMut {
    //         positions: Box::new(self.get_neighbor_positions(x, y).into_iter()),
    //         grid: &mut self
    //     }
    // }

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

    // pub fn pattern_mut<P: Pattern>(&self, x: usize, y: usize, p: P) -> PatternIter {
    //     unimplemented!()
    // }

    // pub fn patterns<P: Pattern>(&self, x: usize, y: usize, ps: Vec<P>) -> PatternIter {
    //     unimplemented!()
    // }

    // pub fn patterns_mut<P: Pattern>(&self, x: usize, y: usize, ps: Vec<P>) -> PatternIter {
    //     unimplemented!()
    // }
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
                cells: vec![0u8; 3 * 5]
            }
        );
    }

    #[test]
    fn get_cell_in_grid() {
        let grid = Grid {
            width: 3,
            height: 3,
            cells: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let cell = grid.get((1, 1));
        assert_eq!(cell, Some(&2));
    }

    #[test]
    fn get_mut_cell_in_grid() {
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let mut_cell = grid.get_mut((1, 1));
        assert_eq!(mut_cell, Some(&mut 2));
    }

    #[test]
    fn get_unchecked_cell_in_grid() {
        let grid = Grid {
            width: 3,
            height: 3,
            cells: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
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
            cells: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
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
        assert_eq!(grid.cells, vec![1, 1, 1, 2]);
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
            cells: (0..6).collect(),
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
            cells: (0..4).collect(),
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
            cells: (0..4).collect(),
            width: 2,
            height: 2,
        };

        grid.move_and_leave((1, 0), (0, 0), 10);
        assert_eq!(grid.get((1, 0)), Some(&10));
        assert_eq!(grid.get((0, 0)), Some(&1));
    }
}
