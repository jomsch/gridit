//! A 2D Grid Library which utilizes the fun of Iterators.
//! The entry point is the [Grid] struct.   
//! The Grid has iterators for Rows and Columns and also 
//! for iterators depending on a [Position].
//! E.g get the neighbor cells of a position with [Grid::neighbors] or
//! cells depending of a pattern from a given position with [Grid::pattern].

mod grid;
pub mod iter;
pub mod pattern;
mod step;

pub use grid::{Grid, Position};
pub use step::Step;
pub use iter::PositionsEnumerator;
