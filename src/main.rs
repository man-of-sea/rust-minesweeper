mod board;
mod cell;

use board::{Board};

fn main() {
    let board = Board::new(9, 9, 0);
    let neighbours = board.neighbours(3, 0);
    println!("{:?}", neighbours);
}
