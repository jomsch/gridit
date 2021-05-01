use super::{BoardPiece, Piece, PColor};
use ggez::{graphics, filesystem};
use gridit::{Grid, NeighborIter, PositionsEnumerator, Position, DirectionPattern, PatternIter, Repeat};

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
            .positions()
            .filter_map(|(pos, o)| {
                match o {
                    // Same Color Piece
                    Some(p) if p.pcolor() == self.pcolor() => {
                        None 
                    }
                    //Everything else
                    _ => Some(pos) 
                }
            })
            .collect()
    }

    fn pcolor(&self) -> PColor {
        self.pcolor
    }
}
