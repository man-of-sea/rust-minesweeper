use crate::cell::{Cell, CellState};

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub mines: usize,
    pub cells: Vec<Vec<Cell>>,
    pub mines_placed: bool,
    pub game_over: bool,
    pub won: bool
}

