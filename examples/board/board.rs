use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{
    size, BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, StrokeOptions, Mesh, Rect,
};
use ggez::mint::Point2;

use gridit::{Grid, PositionsEnumerator, Position};

use crate::piece::Piece;

pub const WHITE: Color = Color::new(0.85, 0.85, 0.85, 1.0);
pub const BLACK: Color = Color::new(0.15, 0.15, 0.15, 1.0);
pub const SELECT: Color = Color::new(186./255., 202./255., 68./255., 0.9);
pub const RED: Color = Color::new(186./255., 68./255., 68./255., 1.0);


pub type BoardPiece = Option<Box<dyn Piece>>;

pub struct Board {
    pub grid :Grid<BoardPiece>,
    rect: Rect,
    selected_field: Option<Position>,
}

impl Board {
    pub fn new(grid: Grid<BoardPiece>, xy: (f32, f32), size: f32) -> Self {
        let rect = Rect::new(xy.0, xy.1, size, size);
        Self { grid, rect, selected_field: None }
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

    fn get_grid_position(&self, point: Point2<f32>) -> gridit::Position {
        let rect = self.rect;
        let bp = rect.point();
        let field_size = rect.w/8.0;
        let point = Point2::from([point.x - bp.x, point.y - bp.y]);
        let px = (point.x / field_size) as usize;
        let py = (point.y / field_size) as usize;
        (px, py).into()
    }

    pub fn select_field(&mut self, point: Point2<f32>) {
        let clicked_pos = self.get_grid_position(point);
        let mut piece = self.grid.get_mut_unchecked(clicked_pos);
        match (&piece, self.selected_field) {
            (Some(piece), None) => {
                self.selected_field = Some(clicked_pos);
            },
            (None, Some(pos)) => {
                let selected_piece = self.grid.get_unchecked(pos);
                if let Some(piece) = selected_piece {
                    let pmoves =  piece.possible_moves(&self.grid, pos);
                    if pmoves.contains(&clicked_pos) {
                        self.grid.move_to(pos, clicked_pos);
                    }
                    self.selected_field = None;
                }
            }
            (_, _) => (),
        }
    }

    pub fn unselect_field(&mut self) {
        self.selected_field = None;
    }

}

impl Drawable for Board {
    fn draw(&self, ctx: &mut Context, _ : DrawParam) -> GameResult<()> {
        let (bx, by) = (self.rect.x, self.rect.y);
        let rect_size = self.rect.w / 8.0;

        for (position, piece) in self.grid.iter().positions() {
            let (x, y) = position.into();
            let bg_color = match (x + y) % 2 == 0 {
                true => WHITE,
                false => BLACK,
            };
            let (fx, fy) = (x as f32, y as f32);
            let rect_x = fx * rect_size + bx;
            let rect_y = fy * rect_size + by;
            let rect = Rect::new(rect_x, rect_y, rect_size, rect_size);
            let mrect = Mesh::new_rectangle(
                ctx,
                DrawMode::Fill(FillOptions::default()),
                rect,
                bg_color,
            )?;
            graphics::draw(ctx, &mrect, DrawParam::default())?;

            if self.selected_field == Some(position) {
                let mrect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::Fill(FillOptions::default()),
                    rect,
                    SELECT,
                )?;
                mrect.draw(ctx, DrawParam::default())?;
            }
            if let Some(piece) = piece {
                let img = piece.image();
                let iw = (img.width()/2) as f32; 
                let ih = (img.height()/2) as f32; 
                let rw = rect.w/2.0;
                let rh = rect.h/2.0;
                let dest = [rect.x + rw - iw, rect.y + rh - ih];
                img.draw(ctx, DrawParam::new().dest(dest))?;
            }
        }

        if let Some(select_position) = self.selected_field {
            let field = self.grid.get_unchecked(select_position);
            if let Some(piece) = field {
                let moves = piece.possible_moves(&self.grid, select_position);
                for mv in moves {
                    let (x, y) = mv.into();
                    let (fx, fy) = (x as f32, y as f32);
                    let cx = fx * rect_size + bx;
                    let cy = fy * rect_size + by;
                    let radius: f32 = 30.;
                    let hs = rect_size/2.;
                    let hr = radius/2.;
                    let point: Point2<f32> = [cx + hs, cy + hs].into();

                    let cmesh = Mesh::new_circle(
                        ctx, 
                        DrawMode::Fill(FillOptions::default()),
                        point,
                        radius,
                        1.,
                        RED,
                    )?;
                    cmesh.draw(ctx, DrawParam::default())?;
                }
            }
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
