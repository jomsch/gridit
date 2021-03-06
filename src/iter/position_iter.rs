use crate::grid::Position;

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
        Some((x, y).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn grid_positions_iter() {
        let grid = Grid {
            width: 3,
            height: 2,
            items: vec![0; 3 * 2],
        };

        let mut positions = grid.positions();
        assert_eq!(positions.next(), Some((0, 0).into()));
        assert_eq!(positions.next(), Some((1, 0).into()));
        assert_eq!(positions.next(), Some((2, 0).into()));
        assert_eq!(positions.next(), Some((0, 1).into()));
        assert_eq!(positions.next(), Some((1, 1).into()));
        assert_eq!(positions.next(), Some((2, 1).into()));
        assert_eq!(positions.next(), None);
    }
}
