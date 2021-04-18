use ggez::event::{self, EventHandler};
use ggez::graphics::{
    size, BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, Mesh, Rect,
};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use gridit::Grid;
use gridit::PositionsEnumerator;

const WHITE: Color = Color::new(0.85, 0.85, 0.85, 1.0);
const BLACK: Color = Color::new(0.15, 0.15, 0.15, 1.0);


fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("pawn_example", "")
        .build()
        .expect("Could not create context");

    let mut game = ChessGame::new(&mut ctx);
    event::run(ctx, event_loop, game);
}

#[derive(Clone, Debug)]
struct Piece;

#[derive(Clone, Debug)]
struct Field {
    bg_color: Color,
    default_color: Color,
    piece: Option<Piece>,
}

struct Board(Grid<Field>);

impl Drawable for Board {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let (width, height) = size(&ctx);
        let padding = 50.0;
        let width = width - padding * 2.0;
        let height = height - padding * 2.0;
        let rect_w = width / 8.0;
        let rect_h = height / 8.0;

        for ((x, y), field) in self.0.iter().positions() {
            let (x, y) = (x as f32, y as f32);
            let rect_x = x * rect_w;
            let rect_y = y * rect_h;
            let rect = Rect::new(rect_x+padding, rect_y+padding, rect_w, rect_h);
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

struct ChessGame {
    board: Board,
}

impl ChessGame {
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

        Self { board: Board(grid) }
    }
}

impl EventHandler for ChessGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::WHITE);
        graphics::draw(ctx, &self.board, DrawParam::default());
        graphics::present(ctx)
    }
}
