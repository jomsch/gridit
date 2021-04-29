use ggez::graphics;
use gridit::Position;

pub trait Piece {
    fn image(&self) -> &graphics::Image;
}

mod pawn;

pub use pawn::Pawn;
