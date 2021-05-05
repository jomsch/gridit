use super::{BoardPiece, PColor, Piece};
use ggez::graphics;
use gridit::{PositionsEnumerator, SideStepsPattern, Grid, Position,};

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
        let steps: Vec<(i32, i32)> = vec![
            (1, 2),
            (2, 1),
            (-1, 2),
            (1, -2),
            (2, -1),
            (-2, 1),
            (-1, -2),
            (-2, -1),
        ];
        let pattern = SideStepsPattern::new(steps);
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
