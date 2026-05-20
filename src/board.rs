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

    pub fn place_mines(&mut self, safe_row: usize, safe_col: usize) {
        // Tworzymy wektor współrzędnych pól, bez pierwszego klikniętego pola
        let mut positions: Vec<(usize, usize)> = (0..self.rows)
            .flat_map(|r| (0..self.cols).map(move |c| (r, c)))
            .filter(|&(r, c)| r != safe_row || r != safe_col)
            .collect();

        // Wybieramy losowo mines z nich
        positions.shuffle(&mut rand::thread_rng());
        for &(r, c) in positions.iter().take(self.mines) {
            self.cells[r][c].is_mine = true;
        }
    }
}

