use crate::grid::Position;

pub(crate) struct Positions<I> {
    iter: I,
    next_pos: Box<Fn(&I, Option<Position>) -> Position>,
    prev_position: Option<Position>,
}

trait PositionsEnumerator<I: Iterator> {
    pub fn positions(self) -> Positions<I> {
        Position { iter: self }
    }
}

impl<I: Iterator> Iterator for Positions<I> {
    type Item = (Position, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.next_pos(&self.iter, self.prev_position);
        Some((next_pos, self.iter.next()?))
    }
}
