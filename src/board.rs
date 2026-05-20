use crate::cell::{Cell, CellState};
use rand::seq::SliceRandom;

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
            cells: vec![vec![Cell::new(); cols]; rows],
            mines_placed: false,
            game_over: false,
            won: false,
        }
    }

    pub fn place_mines(&mut self, safe_row: usize, safe_col: usize) {
        // Tworzymy wektor współrzędnych pól, bez pierwszego klikniętego pola
        let mut positions: Vec<(usize, usize)> = (0..self.rows)
            .flat_map(|r| (0..self.cols).map(move |c| (r, c)))
            .filter(|&(r, c)| r != safe_row || c != safe_col)
            .collect();

        // Wybieramy losowo mines z nich
        positions.shuffle(&mut rand::thread_rng());
        for &(r, c) in positions.iter().take(self.mines) {
            self.cells[r][c].is_mine = true;
        }
    }

    // Funkcja pomocnicza zwracająca sąsiadów pola
    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = row as i32 + dr;
                let c = col as i32 + dc;
                if r >= 0 && r < self.rows as i32 && c >= 0 && c < self.cols as i32 {
                    result.push((r as usize, c as usize));
                }
            }
        }

        result
    }
}

