use super::{BoardPiece, Piece, PColor};
use ggez::graphics;
use gridit::{Grid, PositionsEnumerator, Position, DirectionPattern, Repeat};

pub struct Pawn {
    img: graphics::Image,
    pcolor: PColor,
}

impl Pawn {
    pub fn new(pcolor: PColor, img: graphics::Image) -> Self {
        Self {
            pcolor,
            img,
        }
    }
}

impl Piece for Pawn {
    fn image(&self) -> &graphics::Image {
        &self.img
    }

    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position> {
        let m: i32 = match self.pcolor {
            PColor::BLACK => 1,
            PColor::WHITE => -1,
        };

        let pattern = DirectionPattern::new((0, m), Repeat::Once);
        grid.pattern(pos, pattern)
            .grid_positions()
            .filter(|(_, o)| !self.same_pcolor(o))
            .map(|(pos, _)| pos)
            .collect()
    }

    fn pcolor(&self) -> PColor {
        self.pcolor
    }
}
