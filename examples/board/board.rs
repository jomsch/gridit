use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{
    size, BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, StrokeOptions, Mesh, Rect,
};
use ggez::mint::Point2;

use gridit::{Grid, PositionsEnumerator};

pub const WHITE: Color = Color::new(0.85, 0.85, 0.85, 1.0);
pub const BLACK: Color = Color::new(0.15, 0.15, 0.15, 1.0);

#[derive(Clone, Debug, PartialEq)]
pub struct Piece;

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub bg_color: Color,
    pub default_color: Color,
    pub piece: Option<Piece>,
}

impl Field {
    fn activate(&mut self) {
        self.bg_color = Color::from_rgb(186, 202, 68);
    }

    fn deactivate(&mut self) {
        self.bg_color = self.default_color;  
    }
}

pub struct Board {
    pub grid :Grid<Field>,
    rect: Rect,
}

impl Board {
    pub fn new(grid: Grid<Field>, xy: (f32, f32), size: f32) -> Self {
        let rect = Rect::new(xy.0, xy.1, size, size);
        Self { grid, rect }
    }

    pub fn contains_point(&self, point: Point2<f32>) -> bool {
        self.rect.contains(point)
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn on_click(&mut self, point: Point2<f32>) {
        self.reset_board_color();
        let cpos = self.get_grid_position(point);
        let field = self.grid.get_mut_unchecked(cpos.0, cpos.1);
        field.activate();
    }

    fn reset_board_color(&mut self) {
        self.grid.iter_mut()
            .for_each(|f| f.deactivate());
    }

    fn get_grid_position(&self, point: Point2<f32>) -> gridit::Position {
        let rect = self.rect;
        let bp = rect.point();
        let field_size = rect.w/8.0;
        let point = Point2::from([point.x - bp.x, point.y - bp.y]);
        let px = (point.x / field_size) as usize;
        let py = (point.y / field_size) as usize;
        (px, py)
    }
}

impl Drawable for Board {
    fn draw(&self, ctx: &mut Context, _ : DrawParam) -> GameResult<()> {
        let (bx, by) = (self.rect.x, self.rect.y);
        let rect_size = self.rect.w / 8.0;

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
            graphics::draw(ctx, &mrect, DrawParam::default())?;
        }
        let mrect = Mesh::new_rectangle(
            ctx,
            DrawMode::Stroke(StrokeOptions::default()),
            self.rect,
            Color::BLACK,
        )?;
        graphics::draw(ctx, &mrect, DrawParam::default())?;

        Ok(())
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Some(self.rect)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}
