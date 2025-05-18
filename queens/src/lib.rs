#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(Self { file, rank })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let ChessPosition { file, rank } = self.position;
        if file == other.position.file {
            return true;
        }
        if rank == other.position.rank {
            return true;
        }

        if (file - rank).abs() == (other.position.file - other.position.rank).abs() {
            return true;
        }

        if (rank - (8 - file)).abs() == (other.position.rank - (8 - other.position.file)).abs() {
            return true;
        }
        false
    }
}
