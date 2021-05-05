use ggez::graphics::Rect;

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
    Pawn,
}

#[derive(Clone)]
pub(crate) struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
}

pub(crate) fn get_piece_rect(piece: &Piece) -> Rect {
    let src_image_y = match piece.color {
        Color::White => 0.0,
        Color::Black => 0.5,
    };
    let src_image_x = match piece.piece_type {
        PieceType::King => 0.0,
        PieceType::Queen => 1.0 / 6.0,
        PieceType::Bishop => 2.0 / 6.0,
        PieceType::Knight => 3.0 / 6.0,
        PieceType::Rook => 4.0 / 6.0,
        PieceType::Pawn => 5.0 / 6.0,
    };

    Rect::new(src_image_x, src_image_y, 1.0 / 6.0, 0.5)
}
