use ggez::event::{self, EventHandler};
use ggez::graphics::DrawParam;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use gridit::Grid;
use gridit::PositionsEnumerator;

mod board;
use crate::board::*;


fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("pawn_example", "")
        .build()
        .expect("Could not create context");

    let mut game = BoardGame::new(&mut ctx);
    event::run(ctx, event_loop, game);
}

struct BoardGame {
    board: Board,
}

impl BoardGame {
    fn new(_ctx: &mut Context) -> Self {
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

        Self { board: Board::new(grid, (50.0, 50.0), 400.0) }
    }
}

impl EventHandler for BoardGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.25, 0.25, 0.25, 1.0));
        graphics::draw(ctx, &self.board, DrawParam::default());
        graphics::present(ctx)
    }
}
