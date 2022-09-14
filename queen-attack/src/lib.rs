#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }
        Some(Self(rank, file))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let Queen(ChessPosition(rank1, file1)) = self;
        let Queen(ChessPosition(rank2, file2)) = other;
        rank1 == rank2 || file1 == file2 || (rank1 - rank2).abs() == (file1 - file2).abs()
    }
}
