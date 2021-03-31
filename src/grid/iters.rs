use std::iter::{Skip, StepBy};
use super::grid::{Grid, Position};
use super::position::{Positions, PositionsEnumerator};

pub struct PositionsIter {
    pub(crate) len: usize,
    pub(crate) width: usize,
    pub(crate) idx: usize,
}

impl Iterator for PositionsIter {
    type Item = Position; 
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            return None;
        }
        let x = self.idx % self.width;
        let y = self.idx / self.width;
        self.idx += 1;
        Some((x, y))
    }
}

pub struct GridIter<'a, T> {
    pub(crate) grid_iter: std::slice::Iter<'a, T>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid_iter.next()
    }
}

fn next_position<'a, T>(inner: &GridIter<'a, T>, prev_pos: Option<Position>) -> Position {
    let (px, py) = match prev_pos  {
        Some(T) => T,
        None => return (0, 0)
    };
    let (x, y) = match px == (inner.width-1) {
        true => (0, py+1),
        false => (px+1, py)
    };
    (x, y)
}

impl<'a, T: 'static> PositionsEnumerator for GridIter<'a, T> {
    fn positions(self) -> Positions<GridIter<'a, T>> {
        Positions {
            inner: self,
            next_pos: Box::new(next_position),
            prev_position: None,
        }
    }
}

#[cfg(test)]
mod  tests {
    use super::*;

    #[test]
    fn grid_iter_positions() {
        let grid = Grid::new(2, 2, 9usize);
        let mut iter = grid.iter().positions();

        assert_eq!(iter.next(), Some(((0, 0), &9)));
        assert_eq!(iter.next(), Some(((1, 0), &9)));
        assert_eq!(iter.next(), Some(((0, 1), &9)));
        assert_eq!(iter.next(), Some(((1, 1), &9)));
        assert_eq!(iter.next(), None);
    }
}

pub struct GridIterMut<'a, T> {
    pub(crate) grid_iter: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid_iter.next()
    }
}

pub struct RowIter<'a, T> {
    pub(crate) row_iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
    }
}

pub struct RowIterMut<'a, T> {
    pub(crate) row_iter: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for RowIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
    }
}

pub struct ColumnIter<'a, T> {
    pub(crate) idx: usize,
    pub(crate) col: usize,
    pub(crate) grid: &'a Grid<T>,
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.grid.get(self.col, self.idx - 1)
    }
}

pub struct ColumnIterMut<'a, T> {
    // TODO change column_iter to be a more generic type that implements Iterator
    pub(crate) column_iter: StepBy<Skip<GridIterMut<'a, T>>>,
}

impl<'a, T> Iterator for ColumnIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.column_iter.next()
    }
}

pub struct NeighborIter<'a, T> {
    pub(crate) positions: Box<Iterator<Item = Position>>,
    pub(crate) grid: &'a Grid<T>,
}

impl<'a, T> Iterator for NeighborIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.positions.next()?;
        let cell = self.grid.get_unchecked(pos.0, pos.1);
        Some(cell)
    }
}

// pub struct NeighborIterMut<'a,T> {
//     pub(crate) positions: Box<Iterator<Item = (usize, usize)>>,
//     pub(crate) grid: &'a mut Grid<T>,
// }

// impl<'a, T> Iterator for NeighborIterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let pos = self.positions.next()?;
//         let mut cell = self.grid.get_mut_unchecked(pos.0, pos.1);
//         Some(cell)
//     }
// }

pub trait Pattern {
    fn pattern(&self) -> PatternIter;
}

pub struct PatternIter;
