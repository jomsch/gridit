use std::mem;

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize
}


impl<T: Clone> Grid<T> {

    pub fn new(width: usize, height: usize, default_value: T) -> Self
    {
        Self {
            width,
            height,
            cells: vec![default_value; width*height]
        }
    }

    #[inline]
    fn translate(&self, x: usize, y: usize) -> usize {
        x * self.height + y 
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

    pub fn row(&self, y: usize) -> RowIter {
        unimplemented!()
    }

    pub fn row_mut(&self, y: usize) -> RowIter {
        unimplemented!()
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

// impl<T: PartialEq> PartialEq for Grid<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self == other
//     }
// }

pub trait Pattern {
    fn pattern(&self) -> PatternIter;

}

pub struct RowIter;
pub struct ColumnIter;
pub struct NeighborIter;
pub struct PatternIter;


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_grid() {
        let grid = Grid::new(3, 5, 0u8);
        assert_eq!(grid, Grid { width: 3, height: 5, cells: vec![0u8; 3*5] });
    }

    #[test]
    fn get_cell_in_grid() {
        let grid = Grid::new(3, 5, 1u8);
        let cell = grid.get(2, 2);
        assert_eq!(cell, Some(&1));

        let mut grid = grid;
        let mut_cell = grid.get_mut(1, 1);
        assert_eq!(mut_cell, Some(&mut 1));
    }
}
