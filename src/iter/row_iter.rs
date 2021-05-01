use crate::{PositionsEnumerator, Positions, Position};

pub struct RowIter<'a, T> {
    pub(crate) row_iter: std::slice::Iter<'a, T>,
    pub(crate) idx: usize,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
    }
}

impl<'a, T: 'static> PositionsEnumerator for RowIter<'a, T> {
    fn grid_positions(self) -> Positions<Self> {
        Positions {
            next_pos: |inner, prev_pos| {
                match prev_pos {
                    None => (0, inner.idx).into(),
                    Some(p) => (p.x + 1, p.y).into()
                }
            },
            prev_position: None,
            inner: self
        }
    }
}


pub struct RowIterMut<'a, T> {
    pub(crate) row_iter: std::slice::IterMut<'a, T>,
    pub(crate) idx: usize,
}

impl<'a, T> Iterator for RowIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
    }
}

impl <'a, T: 'static> PositionsEnumerator for RowIterMut<'a, T> {
    fn grid_positions(self) -> Positions<Self> {
        Positions {
            next_pos: |inner, prev_pos| {
                match prev_pos {
                    None => (0, inner.idx).into(),
                    Some(p) => (p.x + 1, p.y).into()
                }
            },
            prev_position: None,
            inner: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;
    use super::*;

    #[test]
    fn row_iter_1x2() {
        let grid =  Grid {
            width: 1,
            height: 2,
            cells: vec![0, 1]
        };

        let mut row_iter = grid.row(0);
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row(1);
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn row_iter_2x1() {
        let grid =  Grid {
            width: 2,
            height: 1,
            cells: vec![0, 1]
        };

        let mut row_iter = grid.row(0);
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn row_iter_2x2() {
        let grid = Grid {
            width: 2,
            height: 2,
            cells: vec![0, 0, 1, 1],
        };
        let mut row_iter = grid.row(0);
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), Some(&0));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row(1);
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn row_iter_positions() {
        let grid = Grid {
            width: 4,
            height: 3,
            cells: vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2]
        };

        let mut row_pos = grid.row(1).grid_positions();
        assert_eq!(row_pos.next(), Some(((0, 1).into(), &1)));
        assert_eq!(row_pos.next(), Some(((1, 1).into(), &1)));
        assert_eq!(row_pos.next(), Some(((2, 1).into(), &1)));
        assert_eq!(row_pos.next(), Some(((3, 1).into(), &1)));
        assert_eq!(row_pos.next(), None);
    }

    #[test]
    fn row_iter_mut_1x2() {
        let mut grid =  Grid {
            width: 1,
            height: 2,
            cells: vec![0, 1]
        };

        let mut row_iter = grid.row_mut(0);
        assert_eq!(row_iter.next(), Some(&mut 0));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row_mut(1);
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn row_iter_mut_2x1() {
        let mut grid =  Grid {
            width: 2,
            height: 1,
            cells: vec![0, 1]
        };

        let mut row_iter = grid.row_mut(0);
        assert_eq!(row_iter.next(), Some(&mut 0));
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn row_iter_mut_2x2() {
        let mut grid = Grid {
            width: 2,
            height: 2,
            cells: vec![0, 0, 1, 1],
        };
        let mut row_iter = grid.row_mut(0);
        assert_eq!(row_iter.next(), Some(&mut 0));
        assert_eq!(row_iter.next(), Some(&mut 0));
        assert_eq!(row_iter.next(), None);

        let mut row_iter = grid.row_mut(1);
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn row_iter_mut_positions() {
        let mut grid = Grid {
            width: 4,
            height: 3,
            cells: vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2]
        };

        let mut row_pos = grid.row_mut(1).grid_positions();
        assert_eq!(row_pos.next(), Some(((0, 1).into(), &mut 1)));
        assert_eq!(row_pos.next(), Some(((1, 1).into(), &mut 1)));
        assert_eq!(row_pos.next(), Some(((2, 1).into(), &mut 1)));
        assert_eq!(row_pos.next(), Some(((3, 1).into(), &mut 1)));
        assert_eq!(row_pos.next(), None);

        let mut row_pos = grid.row_mut(0).grid_positions();
        assert_eq!(row_pos.next(), Some(((0, 0).into(), &mut 0)));
        assert_eq!(row_pos.next(), Some(((1, 0).into(), &mut 0)));
        assert_eq!(row_pos.next(), Some(((2, 0).into(), &mut 0)));
        assert_eq!(row_pos.next(), Some(((3, 0).into(), &mut 0)));
        assert_eq!(row_pos.next(), None);
    }
}
