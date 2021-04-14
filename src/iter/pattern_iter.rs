use crate::pattern::{Pattern, Repeat};
use crate::{Grid, Position};

pub struct PatternIter<'a, T> {
    pub(crate) grid: &'a Grid<T>,
    pub(crate) prev_position: Position,
    pub(crate) pattern: Box<dyn Pattern>,
    pub(crate) repeat_count: usize,
}

impl<'a, T> PatternIter<'a, T> {
    fn repeation_done(&self) -> Option<()> {
        match self.pattern.repeat() {
            Repeat::Once if self.repeat_count != 0 => None,
            Repeat::Times(t) if self.repeat_count >= *t => None,
            _ => Some(()), 
        }

    }
}

impl<'a, T> Iterator for PatternIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.repeation_done()?;
        let step = self.pattern.next_step()?;
        let next_position = step.take_step_from_position(self.prev_position)?;
        let (nx, ny) = next_position;
        let cell = self.grid.get(nx, ny)?;
        self.repeat_count += 1;
        self.prev_position = next_position;
        Some(cell)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pattern::{SequencePattern, DirectionPattern};
    use crate::Step;

    // 0, 1, 2, 3
    // 4, 5, 6, 7
    // 8, 9,10,11 
    //12,13,14,15 
    #[test]
    fn pattern_iter_direction_north() {
        let grid = Grid {
            width: 4,
            height:4, 
            cells: (0..16).collect(), 
        };

        let pattern = DirectionPattern::new((0isize, -1), Repeat::TillEnd);
        let mut iter = grid.pattern(2, 3, pattern);
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_direction_east() {
        let grid = Grid {
            width: 4,
            height:4, 
            cells: (0..16).collect(), 
        };

        let pattern = DirectionPattern::new((1isize, 0), Repeat::TillEnd);
        let mut iter = grid.pattern(0, 2, pattern);
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_direction_south_two_times() {
        let grid = Grid {
            width: 4,
            height:4, 
            cells: (0..16).collect(), 
        };

        let pattern = DirectionPattern::new((0isize, 1), Repeat::Times(2));
        let mut iter = grid.pattern(0, 0, pattern);
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn patter_iter_direction_west_once() {
        let grid = Grid {
            width: 4,
            height:4, 
            cells: (0..16).collect(), 
        };

        let pattern = DirectionPattern::new((-1isize, 0), Repeat::Once);
        let mut iter = grid.pattern(2, 3, pattern);
        assert_eq!(iter.next(), Some(&13));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_sequence() {
        let grid = Grid {
            width: 3,
            height:3,
            cells: (0..9).collect(),
        };

        let seq: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (1, 0), (-2, 1), (1, 0), (1, 0), (-2, 1), (1, 0), (1, 0)];
        let pattern = SequencePattern::new(seq.into_iter().map(Step::from));
        let mut iter = grid.pattern(0, 1, pattern);

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_sequence_cross() {
        let grid = Grid {
            width: 3,
            height:3,
            cells: (0..9).collect(),
        };

        let seq: Vec<(isize, isize)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0), (0, 1), (0, -1), (-1, 0), (1, 0)];
        let pattern = SequencePattern::new(seq.into_iter().map(Step::from));
        let mut iter = grid.pattern(1, 1, pattern);

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }
}