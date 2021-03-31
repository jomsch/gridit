use super::{Positions, PositionsEnumerator};
use crate::grid::Position;

fn next_position<'a, T>(inner: &GridIter<'a, T>, prev_pos: Option<Position>) -> Position {
    let (px, py) = match prev_pos {
        Some(xy) => xy,
        None => return (0, 0),
    };
    let (x, y) = match px == (inner.width - 1) {
        true => (0, py + 1),
        false => (px + 1, py),
    };
    (x, y)
}

pub struct GridIter<'a, T> {
    pub(crate) grid_iter: std::slice::Iter<'a, T>,
    pub(crate) width: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid_iter.next()
    }
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

pub struct GridIterMut<'a, T> {
    pub(crate) grid_iter: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.grid_iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Grid;

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
