#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellContent {
    Empty,
    Number(u8),
    Mine,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub state: CellState,
    pub content: CellContent,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: CellState::Hidden,
            content: CellContent::Empty,
        }
    }
}
