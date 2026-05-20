enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

struct Cell {
    is_mine: bool,
    adjacent: u8,
    state: CellState,
}