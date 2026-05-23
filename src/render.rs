use crossterm::{
    cursor,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType}
};
use std::io::{self, Write};
use crate::board::Board;
use crate::cell::CellState;

pub fn draw(board: &Board, cursor_row: usize, cursor_col: usize) {
    let mut stdout = io::stdout();

    queue!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    draw_header(&mut stdout, board);
    draw_board(&mut stdout, board, cursor_row, cursor_col);
    draw_footer(&mut stdout);

    stdout.flush().unwrap();
}

fn draw_header(stdout: &mut io::Stdout, board: &Board) {
    let remaining = board.mines as isize - board.flags_placed() as isize;
    queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print(format!(" mines: {}   ", remaining)),
        ResetColor,
        Print("\r\n\r\n")
    ).unwrap();
}

fn draw_board(stdout: &mut io::Stdout, board: &Board, cursor_row: usize, cursor_col: usize) {
    for r in 0..board.rows {
        queue!(stdout, Print("  ")).unwrap();

        for c in 0..board.cols {
            let cell = &board.cells[r][c];
            let is_cursor = r == cursor_row && c == cursor_col;

            let (symbol, fg, bg) = match &cell.state {
                CellState::Hidden => (
                    "#",
                    Color::DarkGrey,
                    Color::Grey,
                ),
                CellState::Flagged => (
                    "F",
                    Color::Red,
                    Color::Grey,
                ),
                CellState::Revealed if cell.is_mine => (
                    "*",
                    Color::White,
                    Color::Red,
                ),
                CellState::Revealed => {
                    let (sym, fg) = match cell.adjacent {
                        0 => (" ", Color::White),
                        1 => ("1", Color::Blue),
                        2 => ("2", Color::Green),
                        3 => ("3", Color::Red),
                        4 => ("4", Color::DarkBlue),
                        5 => ("5", Color::DarkRed),
                        6 => ("6", Color::Cyan),
                        7 => ("7", Color::Magenta),
                        _ => ("8", Color::DarkGrey),
                    };
                    (sym, fg, Color::White)
                }
            };

            let bg = if is_cursor { Color::Yellow } else { bg };

            queue!(
                stdout,
                SetForegroundColor(fg),
                SetBackgroundColor(bg),
                Print(format!(" {} ", symbol)),
                ResetColor
            ).unwrap()
        }

        queue!(stdout, Print("\r\n")).unwrap();
    }
}

fn draw_footer(stdout: &mut io::Stdout) {
    queue!(
        stdout,
        Print("\r\n"),
        SetForegroundColor(Color::DarkGrey),
        Print("  arrows: move   space: reveal   f: flag    r: restart   q:   quit"),
        ResetColor
    ).unwrap();
}

pub fn draw_message(msg: &str) {
    let mut stdout = io::stdout();
    queue!(
        stdout,
        Print("\r\n\r\n"),
        SetForegroundColor(Color::Yellow),
        Print(format!("  {}", msg)),
        ResetColor
    ).unwrap();
    stdout.flush().unwrap();
}