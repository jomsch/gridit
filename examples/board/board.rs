use ggez::graphics::{
    BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, Mesh, Rect, StrokeOptions,
};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};

use gridit::{Grid, Position, PositionsEnumerator};

use crate::piece::Piece;

pub const WHITE: Color = Color::new(210./255., 171./255.,111./255., 1.0);
pub const BLACK: Color = Color::new(155./255., 111./255.,51./255., 1.0);
pub const SELECT: Color = Color::new(34. / 255., 201. / 255., 220./ 255., 1.0);
pub const HOVER: Color = Color::new(0.5, 0.5, 0.5, 0.7);

pub type BoardPiece = Option<Box<dyn Piece>>;

pub struct Board {
    pub grid: Grid<BoardPiece>,
    rect: Rect,
    selected_field: Option<Position>,
    hover_field: Option<Position>,
}

impl Board {
    pub fn new(grid: Grid<BoardPiece>, xy: (f32, f32), size: f32) -> Self {
        let rect = Rect::new(xy.0, xy.1, size, size);
        Self {
            grid,
            rect,
            selected_field: None,
            hover_field: None,
        }
    }

    pub fn contains_point(&self, point: Point2<f32>) -> bool {
        self.rect.contains(point)
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_grid_position(&self, point: Point2<f32>) -> gridit::Position {
        let rect = self.rect;
        let bp = rect.point();
        let field_size = rect.w / 8.0;
        let point = Point2::from([point.x - bp.x, point.y - bp.y]);
        let px = (point.x / field_size) as usize;
        let py = (point.y / field_size) as usize;
        (px, py).into()
    }

    pub fn select_field(&mut self, point: Point2<f32>) {
        let clicked_pos = self.get_grid_position(point);
        let piece = self.grid.get_unchecked(clicked_pos);
        match (&piece, self.selected_field) {
            (Some(_piece), None) => {
                self.selected_field = Some(clicked_pos);
            }
            (_, Some(pos)) => {
                let selected_piece = self.grid.get_unchecked(pos);
                if let Some(piece) = selected_piece {
                    let pmoves = piece.possible_moves(&self.grid, pos);
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

    pub fn hover_field(&mut self, point: Point2<f32>) {
        let clicked_pos = self.get_grid_position(point);
        if matches!(self.selected_field, Some(pos) if pos == clicked_pos) {
            self.hover_field = None
        } else {
            self.hover_field = Some(clicked_pos);
        }
    }

    pub fn unhover(&mut self) {
        self.hover_field = None;
    }

    pub fn set_field(&mut self, point: Point2<f32>, piece: Box<dyn Piece>) {
        let clicked_pos = self.get_grid_position(point);
        self.grid.set_unchecked(clicked_pos, Some(piece));
    }
}

impl Drawable for Board {
    fn draw(&self, ctx: &mut Context, _: DrawParam) -> GameResult<()> {
        let (bx, by) = (self.rect.x, self.rect.y);
        let rect_size = self.rect.w / 8.0;

        for (position, piece) in self.grid.iter().grid_positions() {
            let (x, y) = position.into();
            let bg_color = match (x + y) % 2 == 0 {
                true => WHITE,
                false => BLACK,
            };
            let (fx, fy) = (x as f32, y as f32);
            let rect_x = fx * rect_size + bx;
            let rect_y = fy * rect_size + by;
            let rect = Rect::new(rect_x, rect_y, rect_size, rect_size);
            let mrect =
                Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), rect, bg_color)?;
            graphics::draw(ctx, &mrect, DrawParam::default())?;

            if self.selected_field == Some(position) {
                let mrect =
                    Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), rect, SELECT)?;
                mrect.draw(ctx, DrawParam::default())?;
            }

            if self.hover_field == Some(position) {
                let mrect =
                    Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), rect, HOVER)?;
                mrect.draw(ctx, DrawParam::default())?;
            }

            if let Some(piece) = piece {
                let img = piece.image();
                let mut drect = rect;
                drect.scale(0.75, 0.75);
                let img_rect = img.dimensions();
                let sx = drect.w/img_rect.w;
                let sy = drect.h/img_rect.h;
                let iw = drect.w / 2.;
                let ih = drect.h / 2.;
                let rw = rect.w / 2.0;
                let rh = rect.h / 2.0;
                let dest = [rect.x + rw - iw, rect.y + rh - ih];
                img.draw(ctx, DrawParam::new().dest(dest).scale([sx, sy]))?;
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
                    let radius: f32 = 15.;
                    let hs = rect_size / 2.;
                    let point: Point2<f32> = [cx + hs, cy + hs].into();

                    let cmesh = Mesh::new_circle(
                        ctx,
                        DrawMode::Fill(FillOptions::default()),
                        point,
                        radius,
                        1.,
                        SELECT,
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

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.rect)
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}
