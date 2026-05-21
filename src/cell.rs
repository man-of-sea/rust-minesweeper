#[derive(Clone, PartialEq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone)]
pub struct Cell {
    pub is_mine: bool,
    pub adjacent: u8,
    pub state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            is_mine: false,
            adjacent: 0,
            state: CellState::Hidden,
        }
    }
}