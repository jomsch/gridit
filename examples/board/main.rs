use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::DrawParam;
use ggez::graphics::Rect;
use ggez::input::mouse;
use ggez::mint::Point2;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use gridit::GridBuilder;

mod board;
mod picker;
mod piece;
use crate::board::*;
use crate::picker::*;
use crate::piece::*;

const BACKGROUND: graphics::Color = graphics::Color::new(100./255., 61./255., 1./255., 1.0);

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = std::path::PathBuf::from(manifest_dir);
    path.push("resources");

    let (mut ctx, event_loop) = ContextBuilder::new("pawn_example", "")
        .add_resource_path(path)
        .build()
        .expect("Could not create context");

    let game = BoardGame::new(&mut ctx);
    event::run(ctx, event_loop, game);
}

struct BoardGame {
    board: Board,
    picker: Picker,
    has_resized: bool,
    hdpi_factor: f32,
    draggin: Option<(graphics::Image, Name, PColor)>,
}

impl BoardGame {
    fn new(ctx: &mut Context) -> Self {
        let items: Vec<BoardPiece> = (0..64).map(|_| None).collect();
        let mut grid = GridBuilder::new().from(items).width(8).height(8).build();

        grid.set_unchecked((4, 4), Some(new_piece(ctx, Name::PAWN, PColor::BLACK)));

        let hdpi_factor = graphics::window(&ctx).scale_factor() as f32;

        Self {
            board: Board::new(grid, (50.0, 50.0), 400.0),
            picker: Picker::new(ctx, Rect::new(550., 50., 200., 700.)),
            has_resized: true,
            hdpi_factor,
            draggin: None,
        }
    }

    fn resize_board(&mut self, ctx: &Context) {
        let hdpi_factor = graphics::window(ctx).scale_factor() as f32;
        let (x, y) = graphics::size(ctx);
        let size = if x >= y { y } else { x * 0.8 };
        let padding = x/16.;
        let draw_rect = Rect::new(
            padding,
            padding,
            size * hdpi_factor - (padding * 2.),
            size * hdpi_factor - (padding * 2.),
        );
        self.board.set_rect(draw_rect);
    }

    fn resize_picker(&mut self, ctx: &Context) {
        let hdpi_factor = graphics::window(ctx).scale_factor() as f32;
        let (x, y) = graphics::size(ctx);
        let height = y;
        let width = x * 0.20;
        let padding = x/16.;

        let draw_rect = Rect::new(
            (x - width) * hdpi_factor,
            padding,
            width * hdpi_factor - padding,
            height * hdpi_factor - (padding * 2.),
        );
        self.picker.set_rect(draw_rect);
    }
}

impl EventHandler for BoardGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mpoint = mouse::position(ctx);
        if self.draggin.is_some() {
            mouse::set_cursor_type(ctx, mouse::CursorIcon::Grabbing);
        } else if self.picker.on_dragable(mpoint) {
            mouse::set_cursor_type(ctx, mouse::CursorIcon::Grab);
        } else {
            mouse::set_cursor_type(ctx, mouse::CursorIcon::Default);
        }

        if self.board.contains_point(mpoint) {
            self.board.hover_field(mpoint);
        } else {
            self.board.unhover();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BACKGROUND);
        let hdpi_factor = graphics::window(&ctx).scale_factor() as f32;
        if self.has_resized || self.hdpi_factor != hdpi_factor {
            self.hdpi_factor = hdpi_factor;
            let (x, y) = graphics::size(&ctx);
            let draw_rect = Rect::new(0.0, 0.0, x * hdpi_factor, y * hdpi_factor);
            graphics::set_screen_coordinates(ctx, draw_rect)?;
            self.resize_board(ctx);
            self.resize_picker(ctx);
            self.has_resized = false;
        }

        graphics::draw(ctx, &self.board, DrawParam::default())?;
        graphics::draw(ctx, &self.picker, DrawParam::default())?;

        if let Some(item) = &self.draggin {
            let mpos = mouse::position(ctx);
            let img = &item.0;
            let img_center_w = (img.width() / 2) as f32;
            let img_h = (img.height()) as f32;
            let dest: Point2<f32> = [(mpos.x - img_center_w), (mpos.y - img_h)].into();
            graphics::draw(ctx, img, DrawParam::default().dest(dest))?;
        }
        graphics::present(ctx)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {
        self.has_resized = true;
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        let mpoint = [x, y].into();
        if self.board.contains_point(mpoint) && self.draggin.is_some() {
            let info = self.draggin.take();
            let (_, name, pcolor) = info.unwrap();
            self.board.set_field(mpoint, new_piece(ctx, name, pcolor));
        } else if self.draggin.is_some() {
            self.draggin = None;
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let mpoint = [x, y].into();
        if self.board.contains_point(mpoint) {
            self.board.select_field(mpoint);
            return;
        }
        if self.picker.contains_point(mpoint) {
            if self.picker.on_dragable(mpoint) {
                let piece_info = self.picker.get_item_at(mpoint);
                self.draggin = Some(piece_info);
            }
        }
        self.board.unselect_field();
    }
}
