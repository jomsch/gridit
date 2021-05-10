//! All patterns and Pattern Trait used for [pattern](crate::Grid::pattern).
use crate::{Step, Position};

/// This trait is there to create pattern for the [PatternIter](crate::iter::PatternIter).
/// The implemntation should only return one variant of Action. 
/// # Variants
/// * `Action::StepFromOrigin(step_x)` if `step_x` steps outside the grid, this Action will be ignored and next_action will be called again.
/// * `Action::Jump(position_x)` if `position_x` is outside the grid, this Action will be ignored and next_action will be called again.
/// this action will be ignored and the nexj
/// # Panics
/// * if different variants of `Action` are returned
/// * if variant Action::StepFromOrigin does not implement `rest_steps`.
/// * if variant Action::Jump does not implement `rest_positions`.
pub trait Pattern {
    /// Returns the next `Action` or None if there are no more `Action`.
    fn next_action(&mut self) -> Option<Action>;

    /// Peeks in the next `Action` and returns it or None if there is no more `Action`.
    // This is needed to to calculate the position for PositionEnumerator.
    fn next_action_peek(&self) -> Option<Action>;

    /// Returns a reference to the `Repeat`.
    fn repeat(&self) -> &Repeat;

    // rest_step and rest_positon are kind of hacks to make 
    // PositionEnumerator work with pattern


    /// Returns the rest of the steps. This must be implemented for `Action::StepFromOrigin`.
    // This is needed to get the correct position in PositionEnumerator
    // It is only needed for StepFromOrigin action and should be used so
    fn rest_steps(&self) -> Option<Vec<Step>> {
        if matches!(self.next_action_peek(), Some(Action::StepFromOrigin(_))) {
            panic!("Action::StepFromOrigin must implement rest_step");
        }
        None
    }

    /// Returns the rest of the positions. This must be implemented for `Action::Jump`.
    // This is needed to get the correct position in PositionEnumerator
    // It is only needed for Jump action and should be used so
    fn rest_positions(&self) -> Option<Vec<Position>> {
        if matches!(self.next_action_peek(), Some(Action::Jump(_))) {
            panic!("Action::Jump must implement rest_positions");
        }
        None
    }
}

/// Movement action to perform.
// For now Patterns should only use one variant per pattern
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Steps to next position from the previous one.
    // Step from previous position
    Step(Step),

    /// Steps to the next position from the original position provided.
    /// The original position does stay the same.
    // Step from origin position, Steps which do not reach into the grid will be ignored
    StepFromOrigin(Step),

    /// Does jump to the position. No previous or original position are are considered.
    // Jump to any position
    Jump(Position),
}


/// How often a pattern is run.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Repeat {
    Once,
    TillEnd,
    Times(usize),
}


/// Steps in only one direction until end or grid or the repeat condition is meet.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DirectionPattern {
    pub(crate) step: Step,
    pub(crate) repeat: Repeat,
}

impl DirectionPattern {
    pub fn new<S: Into<Step>>(step: S, repeat: Repeat) -> Self {
        Self {
            step: step.into(),
            repeat,
        }
    }
}


impl Pattern for DirectionPattern {
    fn next_action(&mut self) -> Option<Action> {
        Some(Action::Step(self.step))
    }

    fn next_action_peek(&self) -> Option<Action> {
        Some(Action::Step(self.step))
    }

    fn repeat(&self) -> &Repeat {
        &self.repeat
    }
}

/// Walks the steps given, until one step leads outside the grid.
pub struct StepsPattern {
    pub(crate) steps: Vec<Step>,
    pub(crate) idx: usize,
}

impl StepsPattern {
    pub fn new<T: Copy + Into<Step>>(steps: Vec<T>) -> Self {
        Self {
            steps: steps.iter().map(|t| (*t).into()).collect(),
            idx: 0,
        }
    }
}

impl Pattern for StepsPattern {
    fn next_action(&mut self) -> Option<Action> {
        self.idx += 1;
        Some(Action::Step(*self.steps.get(self.idx - 1)?))
    }

    fn next_action_peek(&self) -> Option<Action> {
        Some(Action::Step(*self.steps.get(self.idx)?))
    }

    fn repeat(&self) -> &Repeat {
        &Repeat::TillEnd
    }
}

/// A pattern which side steps from the original position.
/// Steps which lead outside the grid are ignored.
pub struct SideStepsPattern {
    pub(crate) steps: Vec<Step>,
    pub(crate) idx: usize,
}

impl SideStepsPattern {
    pub fn new<I>(steps: I) -> Self 
    where 
        I: IntoIterator,
        I::Item: Copy + Into<Step>,
    {
        Self {
            steps: steps.into_iter().map(|t| t.into()).collect(),
            idx: 0,
        }
    }
}

impl Pattern for SideStepsPattern {
    fn next_action(&mut self) -> Option<Action> {
        self.idx += 1;
        Some(Action::StepFromOrigin(*self.steps.get(self.idx - 1)?))
    }

    fn next_action_peek(&self) -> Option<Action> {
        Some(Action::StepFromOrigin(*self.steps.get(self.idx)?))
    }

    fn repeat(&self) -> &Repeat {
        &Repeat::TillEnd
    }

    fn rest_steps(&self) -> Option<Vec<Step>> {
        Some(self.steps[self.idx..].iter().map(|s| *s).collect())
    }
}

/// A pattern which jumps to the given positions.
/// Positions outside the grid are ignored.
pub struct JumpsPattern {
    jumps: Vec<Position>,
    idx: usize,
}

impl JumpsPattern { 
    pub fn new<I>(positions: I) -> Self 
    where 
        I: IntoIterator,
        I::Item: Copy + Into<Position>,
    {
        Self {
            jumps: positions.into_iter().map(|t| t.into()).collect(),
            idx: 0,
        }
    }
}

impl Pattern for JumpsPattern {
    fn next_action(&mut self) -> Option<Action> {
        self.idx +=1;
        Some(Action::Jump(*self.jumps.get(self.idx -1)?))
    }

    fn next_action_peek(&self) -> Option<Action> {
        Some(Action::Jump(*self.jumps.get(self.idx)?))
    }

    fn repeat(&self) -> &Repeat {
        &Repeat::TillEnd
    }

    fn rest_positions(&self) -> Option<Vec<Position>> {
        Some(self.jumps[self.idx..].iter().map(|s| *s).collect())
    }
}

