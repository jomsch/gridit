use crate::Grid;
use crate::iter::GridIterMut;
use std::iter::{StepBy, Skip};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
