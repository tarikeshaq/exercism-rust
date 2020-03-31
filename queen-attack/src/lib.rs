#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (r, f) if r < 0 || r >= 8 || f < 0 || f >= 8 => None,
            _ => Some(ChessPosition { rank, file }),
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.get_row() == other.get_row()
            || self.get_column() == other.get_column()
            || self.on_diagonal(other)
        {
            return true;
        }

        false
    }

    fn on_diagonal(&self, other: &Queen) -> bool {
        (0..8).any(|count| {
            let row1 = self.get_row() + count;
            let row2 = self.get_row() - count;
            let col1 = self.get_column() + count;
            let col2 = self.get_column() - count;
            [(row1, col1), (row1, col2), (row2, col1), (row2, col2)]
                .iter()
                .cloned()
                .any(|(r1, c1)| r1 == other.get_row() && c1 == other.get_column())
        })
    }

    fn get_row(&self) -> i32 {
        self.position.rank
    }

    fn get_column(&self) -> i32 {
        self.position.file
    }
}
