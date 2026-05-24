#[derive(Clone, Copy)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom { rows: usize, cols: usize, mines: usize }
}

impl Difficulty {
    pub fn settings(&self) -> (usize, usize, usize) {
        match self {
            Difficulty::Beginner     => (9,  9,  10),
            Difficulty::Intermediate => (16, 16, 40),
            Difficulty::Expert       => (16, 30, 99),
            Difficulty::Custom { rows, cols, mines } => (*rows, *cols, *mines)
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Difficulty::Beginner      => "Beginner",
            Difficulty::Intermediate  => "Intermediate",
            Difficulty::Expert        => "Expert",
            Difficulty::Custom { .. } => "Custom"
        }
    }
}