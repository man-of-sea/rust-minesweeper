mod board;
mod cell;

use board::{Board};
use cell::{Cell, CellState};

fn main() {
    let mut board = Board::new(9, 9, 9);
    board.reveal(4, 4);
    
    for i in 0..board.rows {
        for j in 0..board.cols {
            if board.cells[i][j].state == CellState::Revealed {
                print!("{:?} ", board.cells[i][j].adjacent);
            }
            else if board.cells[i][j].state == CellState::Hidden {
                print!("# ");
            }
        }
        print!("\n");
    }
}
