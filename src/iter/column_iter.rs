use super::{Positions, PositionsEnumerator};
use crate::iter::GridIterMut;
use crate::Grid;
use std::iter::{Skip, StepBy};

pub struct ColumnIter<'a, T> {
    pub(crate) row_idx: usize,
    pub(crate) col_idx: usize,
    pub(crate) grid: &'a Grid<T>,
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.row_idx += 1;
        self.grid.get((self.col_idx, self.row_idx - 1))
    }
}

impl<'a, T: 'static> PositionsEnumerator for ColumnIter<'a, T> {
    fn grid_positions(self) -> Positions<Self> {
        Positions {
            inner: self,
            prev_position: None,
            next_pos: |inner, _| (inner.col_idx, inner.row_idx).into(),
        }
    }
}

pub struct ColumnIterMut<'a, T> {
    // TODO change column_iter to be a more generic type that implements Iterator
    pub(crate) iter: StepBy<Skip<GridIterMut<'a, T>>>,
    pub(crate) col_idx: usize,
}

impl<'a, T> Iterator for ColumnIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T: 'static> PositionsEnumerator for ColumnIterMut<'a, T> {
    fn grid_positions(self) -> Positions<Self> {
        Positions {
            inner: self,
            prev_position: None,
            next_pos: |inner, prev_pos| match prev_pos {
                None => (inner.col_idx, 0).into(),
                Some(p) => (p.x, p.y + 1).into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_iter() {
        let grid = Grid {
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
    fn column_iter_positions() {
        let grid = Grid {
            width: 2,
            height: 2,
            cells: vec![0, 1, 0, 1],
        };

        let mut col_pos = grid.column(0).grid_positions();
        assert_eq!(col_pos.next(), Some(((0, 0).into(), &0)));
        assert_eq!(col_pos.next(), Some(((0, 1).into(), &0)));
        assert_eq!(col_pos.next(), None);

        let mut col_pos = grid.column(1).grid_positions();
        assert_eq!(col_pos.next(), Some(((1, 0).into(), &1)));
        assert_eq!(col_pos.next(), Some(((1, 1).into(), &1)));
        assert_eq!(col_pos.next(), None);
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
    fn column_iter_mut_positions() {
        let mut grid = Grid {
            width: 2,
            height: 2,
            cells: vec![0, 1, 0, 1],
        };

        let mut col_pos = grid.column_mut(0).grid_positions();
        assert_eq!(col_pos.next(), Some(((0, 0).into(), &mut 0)));
        assert_eq!(col_pos.next(), Some(((0, 1).into(), &mut 0)));
        assert_eq!(col_pos.next(), None);

        let mut col_pos = grid.column_mut(1).grid_positions();
        assert_eq!(col_pos.next(), Some(((1, 0).into(), &mut 1)));
        assert_eq!(col_pos.next(), Some(((1, 1).into(), &mut 1)));
        assert_eq!(col_pos.next(), None);
    }
}
