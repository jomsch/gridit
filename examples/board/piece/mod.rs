use ggez::{graphics, Context};
use gridit::{Position, Grid};
use crate::BoardPiece;

pub trait Piece {
    fn image(&self) -> &graphics::Image;
    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position>;
}

mod pawn;

pub use pawn::Pawn;


pub enum PColor {
    BLACK,
    WHITE
}

impl PColor {
    fn as_prefix(self) -> &'static str{
        match self {
            PColor::BLACK => "black",
            PColor::WHITE => "white",
        }
    }
}

pub enum Name {
    PAWN,
    TEST,
}

pub fn new_piece(ctx: &mut Context, name: Name, pcolor: PColor) -> Box<dyn Piece> {
    let prefix = pcolor.as_prefix();
    Box::new(match name {
        Name::PAWN => {
            Pawn::new(graphics::Image::new(ctx, format!("/{}_pawn.png", prefix)).unwrap())
        }
        Name::TEST => {
            Pawn::new(graphics::Image::new(ctx, format!("/{}_test.png", prefix)).unwrap())
        }
    })
}
