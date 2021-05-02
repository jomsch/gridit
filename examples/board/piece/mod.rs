use ggez::{graphics, Context};
use gridit::{Position, Grid};
use crate::BoardPiece;

pub trait Piece {
    fn image(&self) -> &graphics::Image;
    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position>;
    fn pcolor(&self) -> PColor;

    fn same_pcolor(&self, o: &Option<Box<dyn Piece>>) -> bool {
        matches!(o, Some(p) if p.pcolor() == self.pcolor()) 
    }
}

mod pawn;
mod rook;
mod bishop;

pub use pawn::Pawn;
pub use rook::Rook;
pub use bishop::Bishop;


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
    ROOK,
    BISHOP,
}

pub fn new_piece(ctx: &mut Context, name: Name, pcolor: PColor) -> Box<dyn Piece> {
    let prefix = pcolor.as_prefix();
    match name {
        Name::PAWN => {
            let p = Pawn::new(pcolor,graphics::Image::new(ctx, format!("/{}_pawn.png", prefix)).unwrap());
            Box::new(p)
        }
        Name::ROOK => {
            let p = Rook::new(pcolor,graphics::Image::new(ctx, format!("/{}_rook.png", prefix)).unwrap());
            Box::new(p)
        },
        Name::BISHOP => {
            let p = Bishop::new(pcolor,graphics::Image::new(ctx, format!("/{}_bishop.png", prefix)).unwrap());
            Box::new(p)
        }
    }
}
