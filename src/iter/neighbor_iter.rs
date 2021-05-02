use super::{Positions, PositionsEnumerator};
use crate::grid::{Grid, Position};

pub struct NeighborIter<'a, T> {
    pub(crate) positions: Vec<Position>,
    pub(crate) grid: &'a Grid<T>,
    pub(crate) idx: usize,
}

impl<'a, T> Iterator for NeighborIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.positions.len() {
            return None;
        }
        let pos = self.positions[self.idx];
        self.idx += 1;
        let cell = self.grid.get_unchecked(pos);
        Some(cell)
    }
}

impl<'a, T> PositionsEnumerator for NeighborIter<'a, T> {
    fn grid_positions(self) -> Positions<Self> {
        Positions {
            inner: self,
            next_pos: |inner, _| {
                if inner.idx < inner.positions.len() {
                    return inner.positions[inner.idx];
                } else {
                    // This only happens when inner.next() returns None
                    // But we need to check since idx can be out of bounds.
                    return (0, 0).into();
                }
            },
            prev_position: None,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbor_iter() {
        let grid = Grid {
            width: 3,
            height: 3,
            cells: (0..9).collect(),
        };

        // middle
        let mut neighbors = grid.neighbors((1, 1));
        assert_eq!(neighbors.next(), Some(&0));
        assert_eq!(neighbors.next(), Some(&1));
        assert_eq!(neighbors.next(), Some(&2));
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&6));
        assert_eq!(neighbors.next(), Some(&7));
        assert_eq!(neighbors.next(), Some(&8));

        // top left corner
        let mut neighbors = grid.neighbors((0, 0));
        assert_eq!(neighbors.next(), Some(&1));
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&4));

        // bottom right corner
        let mut neighbors = grid.neighbors((2, 2));
        assert_eq!(neighbors.next(), Some(&4));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&7));

        // bottom mid
        let mut neighbors = grid.neighbors((1, 2));
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&4));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&6));
        assert_eq!(neighbors.next(), Some(&8));
    }

    #[test]
    fn neighbor_iter_positions_3x3() {
        let grid = Grid {
            width: 3,
            height: 3,
            cells: (0..9).collect(),
        };

        let mut neighbor_pos = grid.neighbors((1, 1)).grid_positions();
        assert_eq!(neighbor_pos.next(), Some(((0, 0).into(), &0)));
        assert_eq!(neighbor_pos.next(), Some(((1, 0).into(), &1)));
        assert_eq!(neighbor_pos.next(), Some(((2, 0).into(), &2)));
        assert_eq!(neighbor_pos.next(), Some(((0, 1).into(), &3)));
        assert_eq!(neighbor_pos.next(), Some(((2, 1).into(), &5)));
        assert_eq!(neighbor_pos.next(), Some(((0, 2).into(), &6)));
        assert_eq!(neighbor_pos.next(), Some(((1, 2).into(), &7)));
        assert_eq!(neighbor_pos.next(), Some(((2, 2).into(), &8)));
        assert_eq!(neighbor_pos.next(), None);
    }

    #[test]
    fn neighbor_iter_positions_2x2() {
        let grid = Grid {
            width: 2,
            height: 2,
            cells: (0..4).collect(),
        };

        let mut neighbor_pos = grid.neighbors((1, 1)).grid_positions();
        assert_eq!(neighbor_pos.next(), Some(((0, 0).into(), &0)));
        assert_eq!(neighbor_pos.next(), Some(((1, 0).into(), &1)));
        assert_eq!(neighbor_pos.next(), Some(((0, 1).into(), &2)));
        assert_eq!(neighbor_pos.next(), None);
    }
}
