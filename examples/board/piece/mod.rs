use ggez::{graphics, Context};
use gridit::{Position, Grid};
use crate::BoardPiece;

pub trait Piece {
    fn image(&self) -> &graphics::Image;
    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position>;
    fn pcolor(&self) -> PColor;
}

mod pawn;

pub use pawn::Pawn;


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PColor {
    BLACK,
    WHITE
}

impl PColor {
    fn as_prefix(&self) -> &'static str{
        match self {
            PColor::BLACK => "black",
            PColor::WHITE => "white",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Name {
    PAWN,
}

pub fn new_piece(ctx: &mut Context, name: Name, pcolor: PColor) -> Box<dyn Piece> {
    let prefix = pcolor.as_prefix();
    Box::new(match name {
        Name::PAWN => {
            Pawn::new(pcolor, graphics::Image::new(ctx, format!("/{}_pawn.png", prefix)).unwrap())
        }
    })
}
