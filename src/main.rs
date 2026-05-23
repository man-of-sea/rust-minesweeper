mod board;
mod cell;
mod render;

use board::{Board};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, ClearType}
};
use std::io;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = run();

    execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}

fn run() -> io::Result<()> {
    let (rows, cols, mines) = (9, 9, 10);
    let mut board = Board::new(rows, cols, mines);
    let mut cursor_row = 0;
    let mut cursor_col = 0;

    loop {
        render::draw(&board, cursor_row, cursor_col);

        if board.game_over {
            render::draw_message("You lost! Press q to quit or r to restart.");
        }
        else if board.won {
            render::draw_message("You won! Press q to quit or r to restart.");
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up        => { if cursor_row > 0 { cursor_row -= 1 } },
                    KeyCode::Down      => { if cursor_row < rows - 1 { cursor_row += 1 } },
                    KeyCode::Left      => { if cursor_col > 0 { cursor_col -= 1} },
                    KeyCode::Right     => { if cursor_col < cols - 1 { cursor_col += 1} },
                    KeyCode::Char(' ') => {
                        if !board.game_over && !board.won {
                            board.reveal(cursor_row, cursor_col);
                        }
                    }
                    KeyCode::Char('f') => {
                        if !board.game_over && !board.won {
                            board.toggle_flag(cursor_row, cursor_col);
                        }
                    }
                    KeyCode::Char('r') => {
                        board = Board::new(rows, cols, mines);
                        cursor_row = 0;
                        cursor_col = 0;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
