mod grid;
mod iter;
mod pattern;
mod step;

pub use grid::{Grid, Position};
pub use iter::*;
pub use pattern::*;
pub use step::Step;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
