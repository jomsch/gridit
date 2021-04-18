use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{
    size, BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, Mesh, Rect,
};

use gridit::Grid;
use gridit::PositionsEnumerator;

pub const WHITE: Color = Color::new(0.85, 0.85, 0.85, 1.0);
pub const BLACK: Color = Color::new(0.15, 0.15, 0.15, 1.0);

#[derive(Clone, Debug)]
pub struct Piece;

#[derive(Clone, Debug)]
pub struct Field {
    pub bg_color: Color,
    pub default_color: Color,
    pub piece: Option<Piece>,
}

pub struct Board {
    pub grid :Grid<Field>,
    pub xy: (f32, f32),
    pub size: f32,
}

impl Board {
    pub fn new(grid: Grid<Field>, xy: (f32, f32), size: f32) -> Self {
        Self { grid, xy, size }
    }
}

impl Drawable for Board {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let (bx, by) = self.xy;
        let rect_size = self.size / 8.0;

        for ((x, y), field) in self.grid.iter().positions() {
            let (x, y) = (x as f32, y as f32);
            let rect_x = x * rect_size + bx;
            let rect_y = y * rect_size + by;
            let rect = Rect::new(rect_x, rect_y, rect_size, rect_size);
            let mrect = Mesh::new_rectangle(
                ctx,
                DrawMode::Fill(FillOptions::default()),
                rect,
                field.bg_color,
            )?;
            graphics::draw(ctx, &mrect, DrawParam::default());
        }

        Ok(())
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        let (width, height) = size(&ctx);
        Some(Rect::new(0.0, 0.0, width, height))
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}
