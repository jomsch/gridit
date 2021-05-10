use super::{BoardPiece, PColor, Piece};
use ggez::graphics;
use gridit::pattern::{DirectionPattern, Repeat};
use gridit::{Grid, Position, PositionsEnumerator};

pub struct Queen {
    img: graphics::Image,
    pcolor: PColor,
}

impl Queen {
    pub fn new(pcolor: PColor, img: graphics::Image) -> Self {
        Self { pcolor, img }
    }
}

impl Piece for Queen {
    fn image(&self) -> &graphics::Image {
        &self.img
    }

    fn possible_moves(&self, grid: &Grid<BoardPiece>, pos: Position) -> Vec<Position> {
        let patterns = [
            DirectionPattern::new((0, 1), Repeat::TillEnd),
            DirectionPattern::new((0, -1), Repeat::TillEnd),
            DirectionPattern::new((1, 0), Repeat::TillEnd),
            DirectionPattern::new((-1, 0), Repeat::TillEnd),
            DirectionPattern::new((1, 1), Repeat::TillEnd),
            DirectionPattern::new((-1, -1), Repeat::TillEnd),
            DirectionPattern::new((1, -1), Repeat::TillEnd),
            DirectionPattern::new((-1, 1), Repeat::TillEnd),
        ];

        patterns
            .iter()
            .map(|pattern| {
                let mut prev: &Option<Box<dyn Piece + 'static>> = &None;
                grid.pattern(pos, *pattern)
                    .grid_positions()
                    .take_while(|(_, o)| {
                        if matches!(prev, Some(p) if p.pcolor() != self.pcolor) {
                            return false;
                        }
                        if self.same_pcolor(o) {
                            return false;
                        }
                        prev = &o;
                        true
                    })
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
