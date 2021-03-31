mod builder;
mod grid;
mod iter;

pub use builder::GridBuilder;
pub use grid::Grid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
