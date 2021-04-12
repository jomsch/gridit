mod builder;
mod grid;
mod iter;
mod pattern;
mod step;

pub use builder::GridBuilder;
pub use grid::{Grid, Position};
pub use step::Step;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
