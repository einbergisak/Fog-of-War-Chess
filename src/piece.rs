pub(crate) type Board = Vec<Option<Piece>>;

#[derive(Clone)]
pub(crate) enum Color {
    White,
    Black,
}

#[derive(Clone)]
pub(crate) enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Clone)]
pub(crate) struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
}
