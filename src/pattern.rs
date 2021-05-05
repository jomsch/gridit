use crate::{Step, Position};

pub trait Pattern {
    fn next_action(&mut self) -> Option<Action>;
    fn next_action_peek(&self) -> Option<Action>;
    fn repeat(&self) -> &Repeat;

    // This is needed to get the correct position in PositionEnumerator
    // It is only needed for StepFromOrigin action and should be used so
    fn rest_positions(&self) -> Option<Vec<Step>>{
        if matches!(self.next_action_peek(), Some(Action::StepFromOrigin(_))) {
            panic!("StepFromOrigin must implement rest_positions");
        }
        None
    }
}

// For now Patterns should only use one type of Patterns
pub enum Action {
    // Step from previous position
    Step(Step),
    // Step from origin position, Steps which do not reach into the grid will be ignored
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

    fn rest_positions(&self) -> Option<Vec<Step>> {
        Some(self.steps[self.idx..].iter().map(|s| *s).collect())
    }
}

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
}

