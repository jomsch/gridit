mod grid;
mod iters;
mod builder;
mod position;

pub use grid::Grid;
pub use builder::GridBuilder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
