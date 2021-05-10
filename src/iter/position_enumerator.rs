use crate::grid::Position;

pub struct Positions<I> {
    pub(crate) inner: I,
    pub(crate) next_pos: fn(&I, Option<Position>) -> Position,
    pub(crate) prev_position: Option<Position>,
}

/// Enumerates the positions of the grid.
/// # Example
/// ```
/// # use gridit::Grid;
/// use gridit::PositionsEnumerator;
/// let grid = Grid::from(vec![1, 2, 1, 2], 2, 2);
/// let mut iter = grid.iter()
///                 .grid_positions()
///                 .filter(|(_, item)| **item == 1);
///
/// assert_eq!(iter.next(), Some(((0, 0).into(), &1)));
/// assert_eq!(iter.next(), Some(((0, 1).into(), &1)));
/// assert_eq!(iter.next(), None);
/// ```
pub trait PositionsEnumerator
where
    Self: Sized,
{
    fn grid_positions(self) -> Positions<Self>;
}

impl<I: Iterator> Iterator for Positions<I> {
    type Item = (Position, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = (self.next_pos)(&self.inner, self.prev_position);
        self.prev_position = Some(next_pos);
        Some((next_pos, self.inner.next()?))
    }
}
