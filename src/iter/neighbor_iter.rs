use crate::grid::{Grid, Position};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbor_iter() {
        let mut grid = Grid {
            width: 3,
            height: 3,
            cells: (0..9).collect(),
        };

        // middle
        let mut neighbors = grid.neighbors(1, 1);
        assert_eq!(neighbors.next(), Some(&0));
        assert_eq!(neighbors.next(), Some(&1));
        assert_eq!(neighbors.next(), Some(&2));
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&6));
        assert_eq!(neighbors.next(), Some(&7));
        assert_eq!(neighbors.next(), Some(&8));

        // top left corner
        let mut neighbors = grid.neighbors(0, 0);
        assert_eq!(neighbors.next(), Some(&1));
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&4));

        // bottom right corner
        let mut neighbors = grid.neighbors(2, 2);
        assert_eq!(neighbors.next(), Some(&4));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&7));

        // bottom mid
        let mut neighbors = grid.neighbors(1, 2);
        assert_eq!(neighbors.next(), Some(&3));
        assert_eq!(neighbors.next(), Some(&4));
        assert_eq!(neighbors.next(), Some(&5));
        assert_eq!(neighbors.next(), Some(&6));
        assert_eq!(neighbors.next(), Some(&8));
    }
}
