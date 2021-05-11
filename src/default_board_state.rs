use crate::piece::{Board, Color::*, Piece, PieceType::*};

pub(crate) fn generate_default_board() -> Board {
    let mut board = vec![None; 64];
    let piece_order = [
        Rook(false),
        Knight,
        Bishop,
        King(false),
        Queen,
        Bishop,
        Knight,
        Rook(false),
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
            piece_type: Pawn(false),
            color: White,
            index,
        });
    }

    // Black pawns
    for index in 48..56 {
        board[index] = Some(Piece {
            piece_type: Pawn(false),
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
        })
    }
    board
}
