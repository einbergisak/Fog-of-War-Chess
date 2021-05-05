use crate::piece::{Board, Color::*, Piece, PieceType::*};

pub(crate) fn generate_default_board() -> Board {
    let mut board = vec![None; 64];
    let piece_order = [Rook, Knight, Bishop, King, Queen, Bishop, Knight, Rook];
    // White pieces

    for (index, piece_type) in piece_order.iter().enumerate() {
        board[index] = Some(Piece {
            piece_type: piece_type.to_owned(),
            color: White,
        })
    }

    // White pawns
    for i in 8..16 {
        board[i] = Some(Piece {
            piece_type: Pawn,
            color: White,
        });
    }

    // Black pawns
    for i in 48..56 {
        board[i] = Some(Piece {
            piece_type: Pawn,
            color: Black,
        });
    }

    // Nicklas
    board[28] = Some(Piece {
        piece_type: Bishop,
        color: White,
    });

    board[29] = Some(Piece {
        piece_type: Rook,
        color: White,
    });

    board[27] = Some(Piece {
        piece_type: Queen,
        color: White,
    });

    // Black pieces
    for (index, piece_type) in piece_order.iter().enumerate() {
        board[index + 56] = Some(Piece {
            piece_type: piece_type.to_owned(),
            color: Black,
        })
    }
    board
}