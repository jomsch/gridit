use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{
    size, BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, StrokeOptions, Mesh, Rect,
};
use ggez::mint::Point2;

use gridit::{Grid, PositionsEnumerator, Position};

use crate::piece::Piece;

const BACKGROUND: Color = Color::new(0.8, 0.8, 0.8 ,1.0);

pub struct Picker {
    rect: Rect,
}

impl Picker {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}

impl Drawable for Picker {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let mrect = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            self.rect,
            BACKGROUND
        )?;

        mrect.draw(ctx, DrawParam::default())?;
        Ok(())
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        None 
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Some(self.rect) 
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        
    }
}
