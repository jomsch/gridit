use super::{Positions, PositionsEnumerator};
use crate::pattern::{Action, Pattern, Repeat};
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
            }
            Action::StepFromOrigin(step) => {
                let mut next_position = step.take_step_from_position(self.origin_position);
                while next_position.is_none()
                    || self
                        .grid
                        .get(next_position.unwrap_or((0, 0).into()))
                        .is_none()
                {
                    let action = self.pattern.next_action()?;
                    let step = match action {
                        Action::StepFromOrigin(step) => step,
                        _ => panic!("different actions per pattern not supported"),
                    };
                    next_position = step.take_step_from_position(self.origin_position);
                }
                // next_position can only be valid here since we check it in the while loop aboe
                let next_position = next_position.unwrap();
                let cell = self.grid.get(next_position)?;
                self.repeat_count += 1;
                self.prev_position = next_position;
                Some(cell)
            }
            Action::Jump(pos) => {
                let mut pos = pos;
                while !self.grid.is_bounds(pos) {
                    let action = self.pattern.next_action()?;
                    pos = match action {
                        Action::Jump(p) => p,
                        _ => panic!("different actions per pattern not supported"),
                    };
                }
                let cell = self.grid.get_unchecked(pos);
                self.repeat_count += 1;
                self.prev_position = pos;
                Some(cell)
            }
        }
    }
}

impl<'a, T> PositionsEnumerator for PatternIter<'a, T> {
    fn grid_positions(self) -> Positions<Self> {
        Positions {
            prev_position: Some(self.prev_position),
            next_pos: |inner, prev| {
                if let Some(action) = inner.pattern.next_action_peek() {
                    return match action {
                        Action::Step(step) => {
                            //prev can not be None in this case, since we set prev_position
                            step.take_step_from_position(prev.unwrap())
                                .unwrap_or_default()
                        }
                        Action::StepFromOrigin(step) => {
                            let origin_position = inner.origin_position;
                            let mut step = step;
                            let mut next_position = step.take_step_from_position(origin_position);
                            let steps = inner
                                .pattern
                                .rest_steps()
                                .expect("Implement fn rest_positionn");
                            let mut steps = steps.iter();
                            // We can unwrap_or in this while loop since if we are at the end of the
                            // iterator, next is called till it will return None in PositionEnuemrator
                            // Unwarp should always be (0, 0) since then we can leave the loop at the end
                            while next_position.is_none()
                                || !inner.grid.is_bounds(next_position.unwrap_or((0, 0).into()))
                            {
                                step = *steps.next().unwrap_or(&(0, 0).into());
                                next_position = step.take_step_from_position(origin_position);
                            }
                            // This can only be Some since we checka
                            next_position.unwrap()
                        }
                        Action::Jump(pos) => {
                            let mut pos = pos;
                            let rest_positions = inner
                                .pattern
                                .rest_positions()
                                .expect("Please implement rest_position for pattern");
                            let mut positions = rest_positions.iter();
                            // Ignore positions no in grid
                            // We can unwrap_or here since the inner iterator next will return None anyway
                            // if positions is exhausted.
                            while !inner.grid.is_bounds(pos) {
                                pos = *positions.next().unwrap_or(&(0, 0).into());
                            }
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
    use crate::pattern::{DirectionPattern, JumpsPattern, SideStepsPattern, StepsPattern};

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
    fn pattern_iter_steps() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: (0..9).collect(),
        };

        let steps: Vec<(i32, i32)> = vec![
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
        let pattern = StepsPattern::new(steps);
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
    fn pattern_iter_steps_cross() {
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
    fn pattern_iter_steps_positions() {
        let grid = Grid {
            width: 2,
            height: 4,
            items: (0..8).collect(),
        };

        let steps: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let pattern = StepsPattern::new(steps);

        let mut iter = grid.pattern((0, 1), pattern).grid_positions();
        assert_eq!(iter.next(), Some(((1, 1).into(), &3)));
        assert_eq!(iter.next(), Some(((1, 2).into(), &5)));
        assert_eq!(iter.next(), Some(((0, 2).into(), &4)));
        assert_eq!(iter.next(), Some(((0, 1).into(), &2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_sidesteps() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: (0..9).collect(),
        };

        // Every Step greater than 1 should be ignored
        let sidesteps = vec![(-4, -4), (0, -1), (5, 5), (1, 0), (3, 3), (0, 1), (-1, 0)];
        let pattern = SideStepsPattern::new(sidesteps);
        let mut iter = grid.pattern((1, 1), pattern);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_sidesteps_positions() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: (0..9).collect(),
        };

        // Every Step greater than 1 should be ignored
        let sidesteps = vec![(0, -1), (5, 5), (1, 0), (3, 3), (0, 1), (-1, 0)];
        let pattern = SideStepsPattern::new(sidesteps);
        let mut iter = grid.pattern((1, 1), pattern).grid_positions();
        assert_eq!(iter.next(), Some(((1, 0).into(), &1)));
        assert_eq!(iter.next(), Some(((2, 1).into(), &5)));
        assert_eq!(iter.next(), Some(((1, 2).into(), &7)));
        assert_eq!(iter.next(), Some(((0, 1).into(), &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_jumps() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: (0..9).collect(),
        };

        let jumps = vec![(4, 2), (2, 2), (1, 1), (0, 0), (5, 5), (32, 32), (1, 0)];
        let pattern = JumpsPattern::new(jumps);
        let mut iter = grid.pattern((0, 0), pattern);
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pattern_iter_jumps_positions() {
        let grid = Grid {
            width: 3,
            height: 3,
            items: (0..9).collect(),
        };

        let jumps = vec![(4, 2), (2, 2), (1, 1), (0, 0), (5, 5), (32, 32), (1, 0)];
        let pattern = JumpsPattern::new(jumps);
        let mut iter = grid.pattern((0, 0), pattern).grid_positions();
        assert_eq!(iter.next(), Some(((2, 2).into(), &8)));
        assert_eq!(iter.next(), Some(((1, 1).into(), &4)));
        assert_eq!(iter.next(), Some(((0, 0).into(), &0)));
        assert_eq!(iter.next(), Some(((1, 0).into(), &1)));
        assert_eq!(iter.next(), None);
    }
}
