use crate::{Step, Position};

pub trait Pattern {
    fn next_action(&mut self) -> Option<Action>;
    fn next_action_peek(&self) -> Option<Action>;
    fn repeat(&self) -> &Repeat;
}

pub enum Action {
    // Step from previous position
    Step(Step),
    // Step from origin position
    StepFromOrigin(Step),
    // Jump to any position
    Jump(Position),
}

#[derive(Copy, Clone, Debug)]
pub enum Repeat {
    Once,
    TillEnd,
    Times(usize),
}

#[derive(Copy, Clone, Debug)]
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

