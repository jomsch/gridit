mod grid;
pub mod iter;
pub mod pattern;
mod step;

pub use grid::{Grid, Position};
pub use step::Step;
pub use iter::PositionsEnumerator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
