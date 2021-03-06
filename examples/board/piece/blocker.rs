use super::{BoardPiece, PColor, Piece};
use ggez::graphics;
use gridit::{Grid, Position, PositionsEnumerator};

pub struct Blocker {
    img: graphics::Image,
    pcolor: PColor,
}

impl Blocker {
    pub fn new(pcolor: PColor, img: graphics::Image) -> Self {
        Self { pcolor, img }
    }
}

impl Piece for Blocker {
    fn image(&self) -> &graphics::Image {
        &self.img
    }

    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position> {
        grid.iter()
            .grid_positions()
            .filter(|(p, o)| self.same_pcolor(o) && pos != *p)
            .map(|(p, _)| {
                grid.neighbors(p)
                    .grid_positions()
                    .filter(|(_, o)| o.is_none())
                    .map(|(p, _)| p)
                    .collect::<Vec<Position>>()
            })
            .flatten()
            .collect()
    }

    fn pcolor(&self) -> PColor {
        self.pcolor
    }
}
