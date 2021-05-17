use crate::piece::{Board, Piece, PieceColor::*, PieceType};

pub(crate) fn generate_default_board() -> Board {
    let mut board = vec![None; 64];
    let piece_order = [
        PieceType::Rook(false),
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::King(false),
        PieceType::Queen,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook(false),
    ];
    // White pieces

    for (index, piece_type) in piece_order.iter().enumerate() {
        board[index] = Some(Piece {
            piece_type: piece_type.to_owned(),
            color: White,
            index,
        })
    }

    // White pawns
    for index in 8..16 {
        board[index] = Some(Piece {
            piece_type: PieceType::Pawn(false),
            color: White,
            index,
        });
    }

    // Black pawns
    for index in 48..56 {
        board[index] = Some(Piece {
            piece_type: PieceType::Pawn(false),
            color: Black,
            index,
        });
    }

    // Black pieces
    for (index, piece_type) in piece_order.iter().enumerate() {
        board[index + 56] = Some(Piece {
            piece_type: piece_type.to_owned(),
            color: Black,
            index: index + 56,
        });
    }

    board
}
