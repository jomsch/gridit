use super::Grid;
use std::iter::{Skip, StepBy};

pub struct GridIter<'a, T> {
    pub(crate) grid_iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid_iter.next()
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
    pub(crate) positions: Box<Iterator<Item = (usize, usize)>>,
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

pub struct NeighborIterMut;

pub trait Pattern {
    fn pattern(&self) -> PatternIter;
}

pub struct PatternIter;
