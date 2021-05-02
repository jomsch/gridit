use super::{BoardPiece, Piece, PColor};
use ggez::graphics;
use gridit::{Grid, PositionsEnumerator, Position};

pub struct King {
    img: graphics::Image,
    pcolor: PColor,
}

impl King {
    pub fn new(pcolor: PColor, img: graphics::Image) -> Self {
        Self {
            pcolor,
            img,
        }
    }
}

impl Piece for King {
    fn image(&self) -> &graphics::Image {
        &self.img
    }

    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position> {
        grid.neighbors(pos)
            .grid_positions()
            .filter(|(_, o)| !self.same_pcolor(o))
            .map(|(pos, _)| pos)
            .collect()
    }

    fn pcolor(&self) -> PColor {
        self.pcolor
    }
}
