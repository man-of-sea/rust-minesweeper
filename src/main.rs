mod board;
mod cell;
mod render;
mod difficulty;

use difficulty::Difficulty;
use board::{Board};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal
};
use std::io::{self, Write};

fn main() {
    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide).unwrap();

    std::panic::set_hook(Box::new(|panic_info| {
        execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
        eprintln!("{}", panic_info);
    }));

    let _ = run();

    execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn pick_custom_difficulty() -> io::Result<Difficulty> {
    let rows = prompt_number("\n  Rows (5 - 30): ", 5, 30)?;
    let cols = prompt_number("  Cols (5 - 50): ", 5, 50)?;

    let max_mines = (rows * cols) - 1;
    let mines = prompt_number(&format!("  Mines (1 - {}): ", max_mines), 1, max_mines)?;

    Ok(Difficulty::Custom { rows, cols, mines })
}

fn prompt_number(label: &str, min: usize, max: usize) -> io::Result<usize> {
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), cursor::Show)?;

    loop {
        print!("{}", label);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(n) if n >= min && n <= max => {
                terminal::enable_raw_mode()?;
                execute!(io::stdout(), cursor::Hide)?;
                return Ok(n);
            }
            _ => {
                println!("  Please enter a number between {} and {}.", min, max);
            }
        }
    }
}

fn pick_difficulty() -> io::Result<Difficulty> {
    let options = vec![
        Difficulty::Beginner, 
        Difficulty::Intermediate, 
        Difficulty::Expert, 
        Difficulty::Custom { rows: 0, cols: 0, mines: 0 }
    ];
    let mut selected = 0;

    loop {
        render::draw_menu(&options, selected);

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up     => { if selected > 0 { selected -= 1} },
                    KeyCode::Down   => { if selected < options.len() - 1 { selected += 1 } },
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if selected == 3 {
                            return pick_custom_difficulty();
                        }
                        else {
                            return Ok(options[selected])
                        }
                    },
                    KeyCode::Char('q') => return Err(io::Error::new(io::ErrorKind::Interrupted, "quit")),
                    _ => {}
                }
            }
        }
    }
}

fn run() -> io::Result<()> {
    loop {
        let difficulty = pick_difficulty()?;
        let (rows, cols, mines) = difficulty.settings();
        let mut board = Board::new(rows, cols, mines);
        let mut cursor_row = 0;
        let mut cursor_col = 0;

        loop {
            render::draw(&board, cursor_row, cursor_col);

            if board.game_over {
                render::draw_message("You lost! Press q to quit, r to restart or m to exit to menu.");
            }
            else if board.won {
                render::draw_message("You won! Press q to quit, r to restart or m to exit to menu.");
            }

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Err(io::Error::new(io::ErrorKind::Interrupted, "quit")),
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
                        KeyCode::Char('m') => break,
                        _ => {}
                    }
                }
            }
        }
    }
}
