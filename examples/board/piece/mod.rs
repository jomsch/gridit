use ggez::graphics;
use gridit::{Position, Grid};
use crate::BoardPiece;

pub trait Piece {
    fn image(&self) -> &graphics::Image;
    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position>;
}

mod pawn;

pub use pawn::Pawn;
