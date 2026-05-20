pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

pub struct Cell {
    pub is_mine: bool,
    pub adjacent: u8,
    pub state: CellState,
}

