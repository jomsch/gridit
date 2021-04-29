use super::Position;
// Utility Enum for storing Negative(N) and Positive(P) as usize
#[derive(Copy, Clone, Debug)]
pub(crate) enum N {
    //Negative Number
    N(usize),
    //Positive Number
    P(usize),
}

impl N {
    fn get_number(&self) -> usize {
        match self {
            N::N(n) => *n,
            N::P(n) => *n,
        }
    }

    fn from_isize(n: isize) -> Self {
        if n < 0 {
            return N::N((n*-1) as usize);
        }
        N::P(n as usize)
    }

    fn from_i32(n: i32) -> Self {
        if n < 0 {
            return N::N((n*-1) as usize);
        }
        N::P(n as usize)
    }

    pub(crate) fn checked_add_sub(&self, n: usize) -> Option<usize> {
        Some(
        match self {
            N::N(pn) => n.checked_sub(*pn)?,
            N::P(pn) => n.checked_add(*pn)?,
        }
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Step {
    x: N, 
    y: N,
}

impl Step {
    pub fn new(x: usize, y: usize) -> Self {
        Step { x: N::P(x), y: N::P(y) }
    }

    pub fn negate_x(mut self) -> Self {
        self.x = N::N(self.x.get_number());
        self
    }

    pub fn negate_y(mut self) -> Self {
        self.y = N::N(self.y.get_number());
        self
    }

    pub(crate) fn take_step_from_position(&self, pos: Position) -> Option<Position> {
        let x = self.x.checked_add_sub(pos.x)?;
        let y = self.y.checked_add_sub(pos.y)?;
        Some((x, y).into())
    }
}

// TODO Create a better impl for all Numbers T -> (T, T)
impl From<(usize, usize)> for Step {
    fn from((x, y): (usize, usize)) -> Self {
        Step::new(x, y) 
    }
}

// TODO Create a better impl for all Numbers T -> (T, T) where T: Neg or something likes this
impl From<(isize, isize)> for Step {
    fn from((x, y): (isize, isize)) -> Self {
        Step {
            x: N::from_isize(x),
            y: N::from_isize(y),
        }
    }
}

impl From<(i32, i32)> for Step {
    fn from((x, y): (i32, i32)) -> Self {
        Step {
            x: N::from_i32(x),
            y: N::from_i32(y),
        }
    }
}
