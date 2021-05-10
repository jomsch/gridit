use crate::BoardPiece;
use ggez::{graphics, Context};
use gridit::{Grid, Position};

mod bishop;
mod blocker;
mod giraffe;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub use bishop::Bishop;
pub use blocker::Blocker;
pub use giraffe::Giraffe;
pub use king::King;
pub use knight::Knight;
pub use pawn::Pawn;
pub use queen::Queen;
pub use rook::Rook;

pub trait Piece {
    fn image(&self) -> &graphics::Image;
    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position>;
    fn pcolor(&self) -> PColor;

    fn same_pcolor(&self, o: &Option<Box<dyn Piece>>) -> bool {
        matches!(o, Some(p) if p.pcolor() == self.pcolor())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PColor {
    BLACK,
    WHITE,
}

impl PColor {
    fn as_prefix(&self) -> &'static str {
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
    KNIGHT,
    QUEEN,
    KING,
    BLOCKER,
    GIRAFFE,
}

pub fn new_piece(ctx: &mut Context, name: Name, pcolor: PColor) -> Box<dyn Piece> {
    let prefix = pcolor.as_prefix();
    match name {
        Name::PAWN => {
            let p = Pawn::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_pawn.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
        Name::ROOK => {
            let p = Rook::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_rook.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
        Name::BISHOP => {
            let p = Bishop::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_bishop.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
        Name::KNIGHT => {
            let p = Knight::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_knight.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
        Name::QUEEN => {
            let p = Queen::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_queen.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
        Name::KING => {
            let p = King::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_king.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
        Name::BLOCKER => {
            let p = Blocker::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_t.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
        Name::GIRAFFE => {
            let p = Giraffe::new(
                pcolor,
                graphics::Image::new(ctx, format!("/{}_giraffe.png", prefix)).unwrap(),
            );
            Box::new(p)
        }
    }
}
