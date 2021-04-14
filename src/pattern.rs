use crate::Step;

pub trait Pattern {
    fn next_step(&mut self) -> Option<Step>;
    fn next_step_peek(&self) -> Option<Step>;
    fn repeat(&self) -> &Repeat;
}

#[derive(Clone, Debug)]
pub enum Repeat {
    Once,
    TillEnd,
    Times(usize),
}

#[derive(Clone, Debug)]
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
    fn next_step(&mut self) -> Option<Step> {
        Some(self.step)
    }

    fn next_step_peek(&self) -> Option<Step> {
        Some(self.step)
    }

    fn repeat(&self) -> &Repeat {
        &self.repeat
    }

}


pub struct SequencePattern {
    pub(crate) steps: Vec<Step>,
    pub(crate) idx: usize,
}

impl SequencePattern {
    pub fn new<T: Copy + Into<Step>>(sequence: Vec<T>) -> Self 
    {
        Self { 
            steps: sequence.iter().map(|t| (*t).into()).collect(), 
            idx: 0 
        }
    }
}

impl Pattern for SequencePattern {
    fn next_step(&mut self) -> Option<Step> {
        self.idx += 1;
        Some(*self.steps.get(self.idx -1)?)
    }

    fn next_step_peek(&self) -> Option<Step> {
        Some(*self.steps.get(self.idx)?)
    }

    fn repeat(&self) -> &Repeat {
        &Repeat::TillEnd
    }
}
