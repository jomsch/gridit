#[derive(Clone, Debug)]
pub enum Direction {
    YP,
    YN,
    XN,
    XP,
    All,
    XAll,
    YAll,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub(crate) step: (usize, usize),
    pub(crate) directions: Vec<Direction>, 
}

impl Pattern {
    pub fn new(step: (usize, usize), directions: impl Into<Vec<Direction>>) -> Self {
        let directions = directions.into();
        if directions.is_empty() {
            panic!("Can't build pattern without direction");
        }
        Pattern {
            step,
            directions: directions.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PatternBuilder {
    pub(crate) step: (usize, usize),
    pub(crate) directions: Vec<Direction>,
}

impl PatternBuilder {
    pub fn new(step_x: usize, step_y: usize) -> Self {
        PatternBuilder {
            step: (step_x, step_y),
            directions: Vec::new(),
        }
    }

    pub fn add_direction(mut self, direction: Direction) -> Self {
        self.directions.push(direction);
        self
    }

    
    pub fn add_directions(mut self, directions: &[Direction]) -> Self {
        self.directions.extend_from_slice(directions);
        self
    }

    pub fn build(self) -> Pattern {
        if self.directions.is_empty() {
            panic!("Can't build Pattern without direction");
        }
        Pattern {
            step: self.step,
            directions: self.directions,
        }
    }
}
