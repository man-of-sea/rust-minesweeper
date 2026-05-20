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

impl Board {
    pub fn new(rows: usize, cols: usize, mines: usize) -> Self {
        Board {
            rows,
            cols,
            mines,
            cells: vec![vec![Cell:new(); cols]; rows],
            mines_placed: false,
            game_over: false,
            won: false,
        }
    }
}

