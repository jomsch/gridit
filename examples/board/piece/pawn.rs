use super::{BoardPiece, PColor, Piece};
use ggez::graphics;
use gridit::{Grid, Position, PositionsEnumerator};
use gridit::pattern::{DirectionPattern, Repeat};

pub struct Pawn {
    img: graphics::Image,
    pcolor: PColor,
}

impl Pawn {
    pub fn new(pcolor: PColor, img: graphics::Image) -> Self {
        Self { pcolor, img }
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
        let mut front: Vec<Position> = grid.pattern(pos, pattern)
            .grid_positions()
            .filter(|(_, o)| o.is_none())
            .map(|(pos, _)| pos)
            .collect();

        let mut sidesteps: Vec<Position> = [(-1, m), (1, m)].iter()
            .map(|s| {
                let pattern = DirectionPattern::new(*s, Repeat::Once);
                grid.pattern(pos, pattern)
                    .grid_positions()
                    .filter(|(_, o)| matches!(o, Some(p) if p.pcolor() != self.pcolor()))
                    .map(|(pos, _)| pos)
                    .collect::<Vec<Position>>()
            })
            .flatten()
            .collect();
        sidesteps.append(&mut front);
        sidesteps
            
    }

    fn pcolor(&self) -> PColor {
        self.pcolor
    }
}
