use super::{BoardPiece, PColor, Piece};
use ggez::graphics;
use gridit::{DirectionPattern, Grid, Position, PositionsEnumerator, Repeat};

pub struct Knight {
    img: graphics::Image,
    pcolor: PColor,
}

impl Knight {
    pub fn new(pcolor: PColor, img: graphics::Image) -> Self {
        Self { pcolor, img }
    }
}

impl Piece for Knight {
    fn image(&self) -> &graphics::Image {
        &self.img
    }

    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position> {
        let steps: [(i32, i32); 8] = [
            (1, 2),
            (2, 1),
            (-1, 2),
            (1, -2),
            (2, -1),
            (-2, 1),
            (-1, -2),
            (-2, -1),
        ];

        steps
            .iter()
            .map(|s| {
                let pattern = DirectionPattern::new(*s, Repeat::Once);
                grid.pattern(pos, pattern)
                    .grid_positions()
                    .filter(|(_, o)| !self.same_pcolor(o))
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
