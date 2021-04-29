use ggez::event::{self, EventHandler, MouseButton};
use ggez::input::mouse;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::mint::Point2;
use ggez::graphics::DrawParam;
use ggez::{filesystem, graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::Rect;
use gridit::Grid;
use gridit::PositionsEnumerator;
use gridit::GridBuilder;

mod board;
mod piece;
mod picker;
use crate::board::*;
use crate::piece::*;
use crate::picker::*;


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
}

impl BoardGame {
    fn new(ctx: &mut Context) -> Self {
        let mut items: Vec<BoardPiece> = (0..64).map(|_|None).collect();
        let mut grid = GridBuilder::new()
            .from(items)
            .width(8)
            .height(8)
            .build();


        let img = graphics::Image::new(ctx, "/black_pawn.png").unwrap();
        grid.set_unchecked((4, 4), Some(Box::new(Pawn::new(img))));

        let hdpi_factor = graphics::window(&ctx).scale_factor() as f32;

        Self { 
            board: Board::new(grid, (50.0, 50.0), 400.0),
            picker: Picker::new(Rect::new(550., 50., 200., 700.)),
            has_resized: true,
            hdpi_factor,
        }
    }

    fn resize_board(&mut self, ctx: &Context) {
        let hdpi_factor = graphics::window(ctx).scale_factor() as f32;
        let (x, y) = graphics::size(ctx);
        let size = if x >= y {
            y
        } else {
            x * 0.8
        };
        let padding = 50.0;
        let draw_rect = Rect::new(padding, padding, size*hdpi_factor-(padding*2.), size*hdpi_factor-(padding*2.));
        self.board.set_rect(draw_rect);
    }

    fn resize_picker(&mut self, ctx: &Context) {
        let hdpi_factor = graphics::window(ctx).scale_factor() as f32;
        let (x, y) = graphics::size(ctx);
        let height = y;
        let width = x * 0.20;
        let padding = 50.0;

        let draw_rect = Rect::new((x-width)*hdpi_factor, padding, width*hdpi_factor-(padding*2.), height*hdpi_factor-(padding*2.));
        dbg!(draw_rect);
        self.picker.set_rect(draw_rect);
         
    }
}

impl EventHandler for BoardGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.25, 0.25, 0.25, 1.0));
        let hdpi_factor = graphics::window(&ctx).scale_factor() as f32;
        if self.has_resized || self.hdpi_factor != hdpi_factor {
            let (x, y) = graphics::size(&ctx);
            let draw_rect = Rect::new(0.0, 0.0, x*hdpi_factor, y*hdpi_factor);
            graphics::set_screen_coordinates(ctx, draw_rect)?;
            self.resize_board(ctx);
            self.resize_picker(ctx);
            self.has_resized = false;
        }

        graphics::draw(ctx, &self.board, DrawParam::default())?;
        graphics::draw(ctx, &self.picker, DrawParam::default())?;
        graphics::present(ctx)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {
        self.has_resized = true;        
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.board.contains_point([x, y].into()) {
            self.board.select_field([x, y].into());
        } else {
            self.board.unselect_field();
        }
    }
}
