use std::mem;

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        Self {
            width,
            height,
            cells: vec![default_value; width * height],
        }
    }

    #[inline]
    fn translate(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    fn is_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
            return Some(&self.cells[idx]);
        }
        None
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
            return Some(&mut self.cells[idx]);
        }
        None
    }

    // TODO: document valid inputs with increased x value but low y value eg : size (3,3) get_unchecked(8,0)
    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        let idx = self.translate(x, y);
        &self.cells[idx]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
            self.cells[idx] = value;
        }
        None
    }

    // TODO: document valid inputs with increased x value but low y value eg : size (3,3) set_unchecked(8,0)
    pub fn set_unchecked(&mut self, x: usize, y: usize, value: T) {
        let idx = self.translate(x, y);
        self.cells[idx] = value;
    }

    // Sets (x, y) to v and returns the old Value of (x, y)
    pub fn replace(&mut self, x: usize, y: usize, value: T) -> Option<T> {
        if self.is_bounds(x, y) {
            let idx = self.translate(x, y);
            let old = mem::replace(&mut self.cells[idx], value);
            return Some(old);
        }
        None
    }

    pub fn row<'a>(&'a self, y: usize) -> RowIter<'a, T> {
        let start_idx = y * self.height;
        let end_idx = start_idx + self.width;

        RowIter {
            idx: 0,
            row: &self.cells[start_idx..end_idx],
        }
    }

     pub fn row_mut<'a>(&'a mut self, y: usize) -> RowIterMut<'a, T> {
        let start_idx = y * self.height;
        let end_idx = start_idx + self.width;

        RowIterMut {
            idx: 0,
            row: &mut self.cells[start_idx..end_idx],
        }
     }

    pub fn column(&self, x: usize) -> ColumnIter {
        unimplemented!()
    }

    pub fn column_mut(&self, x: usize) -> ColumnIter {
        unimplemented!()
    }

    pub fn neighbors(&self, x: usize, y: usize) -> NeighborIter {
        unimplemented!()
    }

    pub fn neighbors_mut(&self, x: usize, y: usize) -> NeighborIter {
        unimplemented!()
    }

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

pub trait Pattern {
    fn pattern(&self) -> PatternIter;
}

pub struct RowIter<'a, T> {
    idx: usize,
    row: &'a [T],
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.row.get(self.idx - 1)
    }
}

pub struct RowIterMut<'a, T> {
    idx: usize,
    row: &'a mut [T],
}

//impl<'a, T> Iterator for RowIterMut<'a, T> {
//    type Item = &'a mut T;
//    fn next(&mut self) -> Option<Self::Item> {
//      if self.idx >= self.row.len() {
//        return None;
//      } 
//      self.idx += 1;
//      Some(&mut self.row[self.idx -1])
//    }
//}

pub struct ColumnIter;
pub struct NeighborIter;
pub struct PatternIter;

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
    fn row_iter() {
        let grid = Grid {
            width: 3,
            height: 3,
            cells: vec![0, 0, 0, 1, 1, 1, 2, 2, 2],
        };
        let mut row_iter = grid.row_mut(0);
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row_mut(1);
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row_mut(2);
        assert_eq!(row_iter.next(), Some(&2));
        assert_eq!(row_iter.next(), Some(&2));
        assert_eq!(row_iter.next(), Some(&2));
        assert_eq!(row_iter.next(), None);
    }
}
