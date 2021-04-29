use super::{BoardPiece, Piece};
use ggez::{graphics, filesystem};
use gridit::{Grid, NeighborIter, PositionsEnumerator, Position};

pub struct Pawn {
    img: graphics::Image,
}

impl Pawn {
    pub fn new(img: graphics::Image) -> Self {
        Self {
            img
        }
    }
}

impl Piece for Pawn {
    fn image(&self) -> &graphics::Image {
        &self.img
    }

    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position> {
        grid.neighbors(pos)
            .positions()
            .map(|(pos, _)| pos)
            .collect()
    }
}
