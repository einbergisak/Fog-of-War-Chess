
pub(crate) type Board = Vec<Vec<Option<Piece>>>;

pub(crate) enum Color {
	White,
	Black
}

pub(crate) enum PieceType {
	King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

pub(crate) struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color
}
