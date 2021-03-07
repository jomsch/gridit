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

    fn translate(&self, x: usize, y: usize) -> usize {
        unimplemented!()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        unimplemented!()
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> T {
        unimplemented!()
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        unimplemented!()
    }

    // Sets (x, y) to v and returns the old Value of (x, y)
    pub fn replace(&mut self, x: usize, y: usize, v: T) -> T {
        unimplemented!()
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

pub trait Pattern {
    fn pattern(&self) -> PatternIter;

}

pub struct RowIter;
pub struct ColumnIter;
pub struct NeighborIter;
pub struct PatternIter;


// [1, 2, 3, 4]
// [2, 2, 0, 0]
// [3, 3, 0, 4]
// [4, 4, 0, 0]
