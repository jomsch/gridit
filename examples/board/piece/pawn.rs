use super::Piece;
use ggez::{graphics, filesystem};

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
}
