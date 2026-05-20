mod board;
mod cell;

use board::{Board};

fn main() {
    let mut board = Board::new(9, 9, 50);
    board.place_mines(4, 4);
    println!("{:?}", board.count_adjacent_mines(4, 4));
}
