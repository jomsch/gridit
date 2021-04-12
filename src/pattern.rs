use crate::Step;

pub trait Pattern {
    fn next_step(&mut self) -> Option<Step>;
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
    pub fn new(step: Step, repeat: Repeat) -> Self {
        Self {
            step,
            repeat,
        }
    }
}

impl Pattern for DirectionPattern {
    fn next_step(&mut self) -> Option<Step> {
        Some(self.step)
    }

    fn repeat(&self) -> &Repeat {
        &self.repeat
    }

}

pub struct SequencePattern {
    pub(crate) steps: Box<dyn Iterator<Item = Step>>,
}

impl SequencePattern {
    pub fn new<I>(sequence: I) -> Self 
    where
        I: IntoIterator<Item = Step> + 'static,
    {
        Self { steps: Box::new(sequence.into_iter()) }
    }
}

impl Pattern for SequencePattern {
    fn next_step(&mut self) -> Option<Step> {
        self.steps.next()   
    }

    fn repeat(&self) -> &Repeat {
        &Repeat::TillEnd
    }
}

