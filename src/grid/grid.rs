use super::iters::*;
use std::iter::{Skip, StepBy};
use std::mem;


// Utility Enum for storing Negative(N) and Positive(P) as usize
#[derive(Clone)]
enum N {
    //Negative Number
    N(usize),
    //Positive Number
    P(usize)
}

pub type Position = (usize, usize);

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
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

impl<T> Grid<T> {
    #[inline]
    fn translate(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    fn is_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
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
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
            return Some(&self.cells[idx]);
        }
        None
    }

    /// Returns a mutable reference to an element at position x,y
    /// or None, if x or y are out of bounds.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
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
    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        let idx = self.translate(x, y);
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
    pub fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut T {
        let idx = self.translate(x, y);
        &mut self.cells[idx]
    }

    /// Sets the element at position x, y to `value`.  
    /// Returns None if x or y is out of bounds,
    /// or () otherwise
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
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
    pub fn set_unchecked(&mut self, x: usize, y: usize, value: T) {
        let idx = self.translate(x, y);
        self.cells[idx] = value;
    }

    /// Sets the element at position x,y to `value`  
    /// returns the old value of x,y
    /// or None if x or y is out of bounds
    pub fn replace(&mut self, x: usize, y: usize, value: T) -> Option<T> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
            let old = mem::replace(&mut self.cells[idx], value);
            return Some(old);
        }
        None
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
        }
    }

    /// Creates an mutable iterator over every element in the grid.
    pub fn iter_mut<'a>(&'a mut self) -> GridIterMut<'a, T> {
        GridIterMut {
            grid_iter: self.cells.iter_mut(),
        }
    }

    /// Creates an iterator over a specific row in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the row is out of bounds.
    pub fn row<'a>(&'a self, y: usize) -> RowIter<'a, T> {
        assert!(self.is_bounds(0, y));
        let start_idx = y * self.height;
        let end_idx = start_idx + self.width;

        RowIter {
            row_iter: self.cells[start_idx..end_idx].iter(),
        }
    }


    /// Creates an mutable iterator over a specific row in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the row is out of bounds.
    pub fn row_mut<'a>(&'a mut self, y: usize) -> RowIterMut<'a, T> {
        assert!(self.is_bounds(0, y));
        let start_idx = y * self.height;
        let end_idx = start_idx + self.width;

        RowIterMut {
            row_iter: self.cells[start_idx..end_idx].iter_mut(),
        }
    }

    /// Creates an iterator over a specific column in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the column is out of bounds.
    pub fn column<'a>(&'a self, x: usize) -> ColumnIter<'a, T> {
        assert!(self.is_bounds(x, 0));
        ColumnIter {
            idx: 0,
            col: x,
            grid: &self,
        }
    }

    /// Creates an mutable iterator over a specific column in the grid.
    ///
    /// # Panics
    ///
    /// Panics if the column is out of bounds.
    pub fn column_mut<'a>(&'a mut self, x: usize) -> ColumnIterMut<'a, T> {
        assert!(self.is_bounds(x, 0));
        let width = self.width;
        let iter = self.iter_mut().skip(x).step_by(width);
        ColumnIterMut { column_iter: iter }
    }


    // Returns every valid neighbor position of x,y
    fn get_neighbor_positions(&self, x: usize, y: usize) -> Vec<Position> {
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

        let valid_positions: Vec<Position> = neighbor_position.iter()
            .filter_map(|(nx, ny)| {
                let x = match nx {
                    N::N(px) => x.checked_sub(*px)?,
                    N::P(px) => x.checked_add(*px)?
                };
                let y = match ny {
                    N::N(py) => y.checked_sub(*py)?,
                    N::P(py) => y.checked_add(*py)?
                };

                if self.get(x, y).is_some() {
                    return Some((x, y));
                }
                None

            }).collect();

        valid_positions
    }

    /// Returns an iterator over every neighbor from position x,y.
    ///
    /// # Panics
    ///
    /// Panics if x or y is out of bounds.
    pub fn neighbors<'a>(&'a self, x: usize, y: usize) -> NeighborIter<'a, T> {
        assert!(self.is_bounds(x, y));
        NeighborIter {
            positions: Box::new(self.get_neighbor_positions(x, y).into_iter()),
            grid: &self,
        }
    }

    // pub fn neighbors_mut<'a>(&'a mut self, x: usize, y: usize) -> NeighborIterMut<'a, T> {
    //     NeighborIterMut {
    //         positions: Box::new(self.get_neighbor_positions(x, y).into_iter()),
    //         grid: &mut self
    //     }
    // }

    pub fn pattern<P: Pattern>(&self, x: usize, y: usize, p: P) -> PatternIter {
        unimplemented!()
    }

    pub fn pattern_mut<P: Pattern>(&self, x: usize, y: usize, p: P) -> PatternIter {
        unimplemented!()
    }

    pub fn patterns<P: Pattern>(&self, x: usize, y: usize, ps: Vec<P>) -> PatternIter {
        unimplemented!()
    }

    pub fn patterns_mut<P: Pattern>(&self, x: usize, y: usize, ps: Vec<P>) -> PatternIter {
        unimplemented!()
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
        let cell = grid.get(1, 1);
        assert_eq!(cell, Some(&2));
    }

    #[test]
    fn get_mut_cell_in_grid() {
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let mut_cell = grid.get_mut(1, 1);
        assert_eq!(mut_cell, Some(&mut 2));
    }

    #[test]
    fn get_unchecked_cell_in_grid() {
        let grid = Grid {
            width: 3,
            height: 3,
            cells: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };
        let cell = grid.get_unchecked(1, 1);
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
        let cell = grid.get_unchecked(3, 2);
    }

    #[test]
    fn set_cell_in_grid() {
        let mut grid = Grid::new(3, 5, 1u8);
        grid.set(2, 2, 2u8);
        let cell = grid.get(2, 2);
        assert_eq!(cell, Some(&2));
    }

    #[test]
    fn set_unchecked_cell_in_grid() {
        let mut grid = Grid::new(3, 5, 1u8);
        grid.set_unchecked(2, 2, 2u8);
        let cell = grid.get(2, 2);
        assert_eq!(cell, Some(&2));
    }

    #[test]
    #[should_panic]
    fn set_unchecked_panic_cell_in_grid() {
        let mut grid = Grid::new(3, 3, 1u8);
        grid.set_unchecked(2, 3, 2u8);
    }

    #[test]
    fn replace_cell_in_grid() {
        let mut grid = Grid::new(2, 2, 1u8);
        let value = grid.replace(1, 1, 2u8);
        assert_eq!(value, Some(1));
        assert_eq!(grid.cells, vec![1, 1, 1, 2]);
    }

    #[test]
    fn grid_positions_iter() {
        let mut grid = Grid {
            width: 3,
            height: 2,
            cells: vec![0; 3*2],
        };


        let mut positions = grid.positions();
        assert_eq!(positions.next(), Some((0, 0)));
        assert_eq!(positions.next(), Some((1, 0)));
        assert_eq!(positions.next(), Some((2, 0)));
        assert_eq!(positions.next(), Some((0, 1)));
        assert_eq!(positions.next(), Some((1, 1)));
        assert_eq!(positions.next(), Some((2, 1)));
        assert_eq!(positions.next(), None);
    }

    #[test]
    fn grid_iter() {
        let grid = Grid {
            width: 3,
            height: 1,
            cells: vec![0, 1, 2],
        };
        let mut grid_iter = grid.iter();
        assert_eq!(grid_iter.next(), Some(&0));
        assert_eq!(grid_iter.next(), Some(&1));
        assert_eq!(grid_iter.next(), Some(&2));
        assert_eq!(grid_iter.next(), None);
    }

    #[test]
    fn grid_iter_mut() {
        let mut grid = Grid {
            width: 3,
            height: 1,
            cells: vec![0, 1, 2],
        };

        let mut grid_iter = grid.iter_mut();
        assert_eq!(grid_iter.next(), Some(&mut 0));
        assert_eq!(grid_iter.next(), Some(&mut 1));
        assert_eq!(grid_iter.next(), Some(&mut 2));
        assert_eq!(grid_iter.next(), None);
    }

    #[test]
    fn row_iter() {
        let grid = Grid {
            width: 3,
            height: 3,
            cells: vec![0, 0, 0, 1, 1, 1, 2, 2, 2],
        };
        let mut row_iter = grid.row(0);
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row(1);
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row(2);
        assert_eq!(row_iter.next(), Some(&2));
        assert_eq!(row_iter.next(), Some(&2));
        assert_eq!(row_iter.next(), Some(&2));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn row_iter_mut() {
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: vec![0, 0, 0, 1, 1, 1, 2, 2, 2],
        };
        let mut row_iter = grid.row_mut(0);
        assert_eq!(row_iter.next(), Some(&mut 0));
        assert_eq!(row_iter.next(), Some(&mut 0));
        assert_eq!(row_iter.next(), Some(&mut 0));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row_mut(1);
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row_mut(2);
        assert_eq!(row_iter.next(), Some(&mut 2));
        assert_eq!(row_iter.next(), Some(&mut 2));
        assert_eq!(row_iter.next(), Some(&mut 2));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn column_iter() {
        let mut grid = Grid {
            width: 2,
            height: 2,
            cells: vec![0, 1, 0, 1],
        };

        let mut col_iter = grid.column(0);
        assert_eq!(col_iter.next(), Some(&0));
        assert_eq!(col_iter.next(), Some(&0));
        assert_eq!(col_iter.next(), None);

        let mut col_iter = grid.column(1);
        assert_eq!(col_iter.next(), Some(&1));
        assert_eq!(col_iter.next(), Some(&1));
        assert_eq!(col_iter.next(), None);
    }

    #[test]
    fn column_iter_mut() {
        let mut grid = Grid {
            width: 2,
            height: 2,
            cells: vec![0, 1, 0, 1],
        };

        let mut col_iter = grid.column_mut(0);
        assert_eq!(col_iter.next(), Some(&mut 0));
        assert_eq!(col_iter.next(), Some(&mut 0));
        assert_eq!(col_iter.next(), None);

        let mut col_iter = grid.column_mut(1);
        assert_eq!(col_iter.next(), Some(&mut 1));
        assert_eq!(col_iter.next(), Some(&mut 1));
        assert_eq!(col_iter.next(), None);
    }

    #[test]
    fn neighbor_iter() {
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: (0..9).collect(),
        };

        // middle
        let mut neighbors = grid.neighbors(1, 1);
        assert_eq!(neighbors.next(), Some(&0));
        assert_eq!(neighbors.next(), Some(&1));
        assert_eq!(neighbors.next(), Some(&2));
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&6));
        assert_eq!(neighbors.next(), Some(&7));
        assert_eq!(neighbors.next(), Some(&8));

        // top left corner
        let mut neighbors = grid.neighbors(0, 0);
        assert_eq!(neighbors.next(), Some(&1));
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&4));

        // bottom right corner
        let mut neighbors = grid.neighbors(2, 2);
        assert_eq!(neighbors.next(), Some(&4));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&7));

        // bottom mid
        let mut neighbors = grid.neighbors(1, 2);
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&4));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&6));
        assert_eq!(neighbors.next(), Some(&8));
    }
}
