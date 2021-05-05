use super::{Positions, PositionsEnumerator};
use crate::pattern::{Pattern, Repeat, Action};
use crate::{Grid, Position};

pub struct PatternIter<'a, T> {
    pub(crate) grid: &'a Grid<T>,
    pub(crate) prev_position: Position,
    pub(crate) pattern: Box<dyn Pattern>,
    pub(crate) repeat_count: usize,
    pub(crate) origin_position: Position,
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
        let action = self.pattern.next_action()?;
        match action {
            Action::Step(step) => {
                let next_position = step.take_step_from_position(self.prev_position)?;
                let cell = self.grid.get(next_position)?;
                self.repeat_count += 1;
                self.prev_position = next_position;
                Some(cell)
            },
            Action::StepFromOrigin(step) => {
                let next_position = step.take_step_from_position(self.origin_position)?;
                let cell = self.grid.get(next_position)?;
                self.repeat_count += 1;
                self.prev_position = next_position;
                Some(cell)
            },
            Action::Jump(pos) => {
                let cell = self.grid.get(pos)?;
                self.repeat_count += 1;
                self.prev_position = pos;
                Some(cell)
            },
        }
    }
}

impl<'a, T> PositionsEnumerator for PatternIter<'a, T> {
    fn grid_positions(self) -> Positions<Self> {
        Positions {
            prev_position: Some(self.prev_position),
            next_pos: |inner, prev| {
                if let Some(action) =  inner.pattern.next_action_peek() {
                    return match action {
                    Action::Step(step) => {
                            //prev can not be None in this case, since we set prev_position
                            step
                                .take_step_from_position(prev.unwrap())
                                .unwrap_or_default()

                    },
                    Action::StepFromOrigin(step) => {
                            //prev can not be None in this case, since we set prev_position
                            let origin = inner.origin_position;
                            step
                                .take_step_from_position(origin)
                                .unwrap_or_default()
                    },
                    Action::Jump(pos) => {
                        pos
                    }
                    };
                } else {
                    // Since we call .next() for the inner later,
                    // we should not do any bound checks here.
                    (0, 0).into()
                }
            },
            inner: self,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pattern::{DirectionPattern, StepsPattern};

    // 0, 1, 2, 3
    // 4, 5, 6, 7
    // 8, 9,10,11
    //12,13,14,15
    #[test]
    fn pattern_iter_direction_north() {
        let grid = Grid {
            width: 4,
            height: 4,
            items: (0..16).collect(),
        };

        let pattern = DirectionPattern::new((0isize, -1), Repeat::TillEnd);
        let mut iter = grid.pattern((2, 3), pattern);
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_direction_east() {
        let grid = Grid {
            width: 4,
            height: 4,
            items: (0..16).collect(),
        };

        let pattern = DirectionPattern::new((1isize, 0), Repeat::TillEnd);
        let mut iter = grid.pattern((0, 2), pattern);
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_direction_south_two_times() {
        let grid = Grid {
            width: 4,
            height: 4,
            items: (0..16).collect(),
        };

        let pattern = DirectionPattern::new((0, 1), Repeat::Times(2));
        let mut iter = grid.pattern((0, 0), pattern);
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn patter_iter_direction_west_once() {
        let grid = Grid {
            width: 4,
            height: 4,
            items: (0..16).collect(),
        };

        let pattern = DirectionPattern::new((-1, 0), Repeat::Once);
        let mut iter = grid.pattern((2, 3), pattern);
        assert_eq!(iter.next(), Some(&13));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_direction_positions() {
        let grid = Grid {
            width: 4,
            height: 4,
            items: (0..15).collect(),
        };

        let pattern = DirectionPattern::new((-1, -1), Repeat::Times(2));
        let mut iter = grid.pattern((3, 3), pattern).grid_positions();
        assert_eq!(iter.next(), Some(((2, 2).into(), &10)));
        assert_eq!(iter.next(), Some(((1, 1).into(), &5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_sequence() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: (0..9).collect(),
        };

        let seq: Vec<(i32, i32)> = vec![
            (0, -1),
            (1, 0),
            (1, 0),
            (-2, 1),
            (1, 0),
            (1, 0),
            (-2, 1),
            (1, 0),
            (1, 0),
        ];
        let pattern = StepsPattern::new(seq);
        let mut iter = grid.pattern((0, 1), pattern);

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
            height: 3,
            items: (0..9).collect(),
        };

        let seq: Vec<(i32, i32)> = vec![
            (0, -1),
            (0, 1),
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0),
        ];
        let pattern = StepsPattern::new(seq);
        let mut iter = grid.pattern((1, 1), pattern);

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

    #[test]
    fn pattern_iter_sequence_positions() {
        let grid = Grid {
            width: 2,
            height: 4,
            items: (0..8).collect(),
        };

        let seq: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let pattern = StepsPattern::new(seq);

        let mut iter = grid.pattern((0, 1), pattern).grid_positions();
        assert_eq!(iter.next(), Some(((1, 1).into(), &3)));
        assert_eq!(iter.next(), Some(((1, 2).into(), &5)));
        assert_eq!(iter.next(), Some(((0, 2).into(), &4)));
        assert_eq!(iter.next(), Some(((0, 1).into(), &2)));
        assert_eq!(iter.next(), None);
    }
}
