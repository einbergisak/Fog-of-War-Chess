use ggez::graphics::Rect;

use crate::piece_movement::{bishop_valid_moves, king_valid_moves, knight_valid_moves, pawn_valid_moves, rook_valid_moves};

pub(crate) type Board = Vec<Option<Piece>>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Debug)]
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

pub(crate) fn get_valid_move_indices(
    board: &Board,
    piece: &Piece,
    piece_source_index: usize,
) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    println!(
        "Piece source index {}, cointains: {:?}",
        piece_source_index, piece
    );
    println!("Found piece at source index {}", piece_source_index);

    match piece.piece_type {
        PieceType::King => {
            indices.append(&mut king_valid_moves(board, piece, piece_source_index))
        }
        PieceType::Queen => {
            // Queen moves as both Rook and Bishop
            indices.append(&mut rook_valid_moves(board, piece, piece_source_index));
            indices.append(&mut bishop_valid_moves(board, piece, piece_source_index));
        }
        PieceType::Rook => {
            // Horizontal + Vertical movement
            indices.append(&mut rook_valid_moves(board, piece, piece_source_index));
        }
        PieceType::Bishop => {
            // Diagonal movement
            indices.append(&mut bishop_valid_moves(board, piece, piece_source_index));
        }
        PieceType::Knight => {
            // Move two steps either vertically or horizontally, then one step in a perpendicular direction.
            indices.append(&mut knight_valid_moves(board, piece, piece_source_index));
        }
        PieceType::Pawn => {
            indices.append(&mut pawn_valid_moves(board, piece, piece_source_index))
        }
    }

    // Returns a list of the valid moves
    indices
}
