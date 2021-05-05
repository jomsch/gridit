use super::{BoardPiece, PColor, Piece};
use ggez::graphics;
use gridit::{DirectionPattern, Grid, Position, PositionsEnumerator, Repeat, Pattern};

pub struct Giraffe {
    img: graphics::Image,
    pcolor: PColor,
}

impl Giraffe {
    pub fn new(pcolor: PColor, img: graphics::Image) -> Self {
        Self { pcolor, img }
    }
}

impl Piece for Giraffe {
    fn image(&self) -> &graphics::Image {
        &self.img
    }

    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position> {
        Vec::new()
    }

    fn pcolor(&self) -> PColor {
        self.pcolor
    }
}
