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

    fn place_mines(&mut self, safe_row: usize, safe_col: usize) {
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

        // Zliczamy ile min sąsiaduje z każdym polem
        for r in 0..self.rows {
            for c in 0..self.cols {
                if !self.cells[r][c].is_mine {
                    self.cells[r][c].adjacent = self.count_adjacent_mines(r, c);
                }
            }
        }

        self.mines_placed = true;
    }

    // Funkcja pomocnicza zliczająca miny sąsiadujące z polem
    fn count_adjacent_mines(&self, row: usize, col: usize) -> u8 {
        self.neighbours(row, col)
            .iter()
            .filter(|&&(r, c)| self.cells[r][c].is_mine)
            .count() as u8
    }

    // Funkcja pomocnicza zwracająca sąsiadów pola
    fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
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

    pub fn reveal(&mut self, row: usize, col: usize) {
        // Przy pierwszym kliknięciu ustawiamy miny, żeby uniknąć natychmiastowej przegranej
        if !self.mines_placed {
            self.place_mines(row, col);
        }

        let cell = &self.cells[row][col];

        // Nie odkrywamy pola, które jest oflagowane lub już odkryte
        if cell.state == CellState::Revealed || cell.state == CellState::Flagged {
            return;
        }

        if cell.is_mine {
            self.reveal_all_mines();
            self.game_over = true;
            return;
        }

        // Odkrywanie klikniętego pola. Może się zdarzyć, że klikniemy na pole nie sąsiadujące z minami.
        // W takim wypadku kontynuujemy odkrywanie, aż wszystkie odkryte pola będą sąsiadowały z minami.
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((row, col));

        while let Some((r, c)) = queue.pop_front() {
            let cell = &mut self.cells[r][c];

            if cell.state == CellState::Revealed {
                continue;
            }
            if cell.state == CellState::Flagged {
                continue;
            }

            cell.state = CellState::Revealed;

            if cell.adjacent == 0 {
                let neighbours = self.neighbours(r, c);
                for (nr, nc) in neighbours {
                    queue.push_back((nr, nc));
                }
            }
        }

        if self.is_won() {
            self.won = true;
        }
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) {
        let cell = &mut self.cells[row][col];

        if cell.state == CellState::Revealed {
            return;
        }
        
        cell.state = match cell.state {
            CellState::Hidden  => CellState::Flagged,
            CellState::Flagged => CellState::Hidden,
            _                  => return,
        }
    }

    pub fn flags_placed(&self) -> usize {
        self.cells.iter().flatten()
            .filter(|c| c.state == CellState::Flagged)
            .count()
    }

    fn is_won(&self) -> bool {
        self.cells.iter().flatten()
            .all(|cell| cell.is_mine || cell.state == CellState::Revealed)
    }

    // Funkcja pomocnicza do odkrycia wszystkich min po przegranej
    fn reveal_all_mines(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.cells[r][c].is_mine {
                    self.cells[r][c].state = CellState::Revealed;
                }
            }
        }
    }
}

