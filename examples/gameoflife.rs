use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{cursor, execute, Command};
use gridit::Grid;
use std::fmt;
use std::io::stdout;
use std::time::Duration;

#[derive(Clone, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

#[derive(Clone, PartialEq)]
struct PodCell {
    pub current: Cell,
    pub future: Cell,
}

impl PodCell {
    fn new(c: Cell) -> Self {
        PodCell {
            current: c.clone(),
            future: c,
        }
    }

    fn update(&mut self) {
        self.current = self.future.clone();
    }
}

impl Cell {
    fn to_char(&self) -> char {
        match *self {
            Cell::Dead => ' ',
            Cell::Alive => '█',
        }
    }
}

fn main() -> crossterm::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), Clear(ClearType::All))?;

    let (rows, cols) = size()?;
    let (row_size, col_size) = (rows as usize, cols as usize);
    //let (row_size, col_size) = (10, 10);
    let (mid_x, mid_y) = ((row_size - 1) / 2, (col_size - 1) / 2);

    let mut grid: Grid<PodCell> = Grid::new(row_size, col_size, PodCell::new(Cell::Dead));
    grid.set_unchecked(mid_x, mid_y - 1, PodCell::new(Cell::Alive));
    grid.set_unchecked(mid_x, mid_y, PodCell::new(Cell::Alive));
    grid.set_unchecked(mid_x, mid_y + 1, PodCell::new(Cell::Alive));

    loop {
        if poll(Duration::from_millis(500))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }

            if let Event::Resize(c, r) = event {
                println!("Rezize to {} {}", r, c);
            }
        }

        for (x, y) in grid.positions() {
            let neighbor_count = grid
                .neighbors(x, y)
                .filter(|c| c.current == Cell::Alive)
                .count();
            let current = grid.get_unchecked(x, y).current.clone();
            let mut cell = grid.get_mut_unchecked(x, y);

            match (&current, neighbor_count) {
                (Cell::Alive, 1) => cell.future = Cell::Dead,
                (Cell::Alive, 2..=3) => cell.future = Cell::Alive,
                (Cell::Dead, 3) => cell.future = Cell::Alive,
                (Cell::Alive, 4..=9) => cell.future = Cell::Dead,
                _ => (),
            }
        }
        grid.iter_mut().for_each(|pod| pod.update());

        let grid_string: String = grid.iter().map(|c| c.current.to_char()).collect();
        execute!(stdout(), GridPrinter(&grid_string))?;
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    execute!(stdout(), cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}

struct GridPrinter<'a>(&'a str);

impl<'a> Command for GridPrinter<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        execute!(stdout(), cursor::MoveTo(0, 0)).expect("Could not move cursor to 0,0");
        f.write_str(&self.0)
    }
}
