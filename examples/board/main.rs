use ggez::event::{self, EventHandler, MouseButton};
use ggez::input::mouse;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::mint::Point2;
use ggez::graphics::DrawParam;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::Rect;
use gridit::Grid;
use gridit::PositionsEnumerator;

mod board;
use crate::board::*;


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("pawn_example", "")
        .build()
        .expect("Could not create context");

    let game = BoardGame::new(&mut ctx);
    event::run(ctx, event_loop, game);
}

struct BoardGame {
    board: Board,
    has_resized: bool,
    hdpi_factor: f32,
}

impl BoardGame {
    fn new(ctx: &mut Context) -> Self {
        let mut grid = Grid::new(
            8,
            8,
            Field {
                default_color: BLACK,
                bg_color: BLACK,
                piece: None,
            },
        );

        for (_, field) in grid.iter_mut().positions().filter(|((x, y), _)| (x + y) % 2 == 0)
        {
            field.default_color = WHITE;
            field.bg_color = WHITE;
        }
        let hdpi_factor = graphics::window(&ctx).scale_factor() as f32;

        Self { 
            board: Board::new(grid, (50.0, 50.0), 400.0),
            has_resized: true,
            hdpi_factor,
        }
    }
}

impl EventHandler for BoardGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mpos = mouse::position(ctx);
        let lbtn_pressed = mouse::button_pressed(ctx,  MouseButton::Left);

        if self.board.contains_point(mpos) && lbtn_pressed {
            println!("{}, {}, MouseButton Pressed: {}", mpos.x, mpos.y, lbtn_pressed);
            let mut brect = self.board.get_rect();
            brect.w += 5.0;
            brect.h += 5.0;
            self.board.set_rect(brect);
        }

    
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.25, 0.25, 0.25, 1.0));
        let hdpi_factor = graphics::window(&ctx).scale_factor() as f32;
        if self.has_resized || self.hdpi_factor != hdpi_factor {
            let hdpi_factor = graphics::window(&ctx).scale_factor() as f32;
            let (x, y) = graphics::size(&ctx);
            let draw_rect = Rect::new(0.0, 0.0, x*hdpi_factor, y*hdpi_factor);
            graphics::set_screen_coordinates(ctx, draw_rect)?;
            self.has_resized = false;
        }

        graphics::draw(ctx, &self.board, DrawParam::default())?;
        graphics::present(ctx)
    }
}
