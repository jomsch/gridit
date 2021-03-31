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

#[cfg(test)]
mod tests {
    use crate::Grid;

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
}
