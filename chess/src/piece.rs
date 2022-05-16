
pub enum PieceType {
    King,
    Queen,
    Pawn,
    Knight,
    Bishop,
    Rook,
}

impl PieceType {
    pub fn curr_type(&self) {
        match self {
            Self::King => println!("King"),
            Self::Queen => println!("Queen"),
            Self::Pawn => println!("Pawn"),
            Self::Knight => println!("Knight"),
            Self::Bishop => println!("Bishop"),
            Self::Rook => println!("Rook"),
        }
    }
}

pub struct Piece {
    is_white: bool,
    piece_type: PieceType,
}

pub trait MovePiece {
    fn move_piece(&self, pos: u32) -> Option<u32>;
}

impl MovePiece for Piece {
    fn move_piece(&self, pos: u32) -> Option<u32> {
        match self.piece_type {
            PieceType::King => Some(1),
            PieceType::Queen => Some(1),
            PieceType::Pawn => Some(1),
            PieceType::Knight => Some(1),
            PieceType::Bishop => Some(1),
            PieceType::Rook => Some(1),
        }
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, is_white: bool) -> Self {
        Self {
            piece_type,
            is_white,
        }
    }
}

