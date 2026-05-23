mod board;
mod cell;
mod render;

use board::{Board};
use crossterm::{
    cursor,
    event,
    execute,
    terminal
};
use std::io;

fn main() {
    let mut board = Board::new(9, 9, 5);
    board.reveal(4, 4);

    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), cursor::Hide).unwrap();

    render::draw(&board, 0, 0);
    event::read().unwrap();

    execute!(io::stdout(), cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}
