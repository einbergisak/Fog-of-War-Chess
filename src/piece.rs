use ggez::graphics::Rect;

use crate::{game::Game, piece_movement::{
    bishop_valid_moves, king_valid_moves, knight_valid_moves, pawn_valid_moves, rook_valid_moves,
}};

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

pub(crate) fn get_valid_move_indices(game: &mut Game, piece: &Piece,
    piece_source_index: usize,
) -> Vec<usize> {
    let board = &game.board;
    println!(
        "Piece source index {}, cointains: {:?}",
        piece_source_index, piece
    );
    println!("Found piece at source index {}", piece_source_index);

    // Returns a list of the valid moves
    match piece.piece_type {
        // King moves one square in any direction
        PieceType::King => king_valid_moves(board, piece, piece_source_index),

        // Queen moves diagonally, vertically or horizontally (Rook + Bishop)
        PieceType::Queen => {
            let mut moves = bishop_valid_moves(board, piece, piece_source_index);
            moves.append(&mut rook_valid_moves(board, piece, piece_source_index));
            moves
        }

        // Rook moves vertically or horizontally
        PieceType::Rook => rook_valid_moves(board, piece, piece_source_index),

        // Bishop moves diagonally
        PieceType::Bishop => bishop_valid_moves(board, piece, piece_source_index),

        // Knight moves two steps either vertically or horizontally, then one step in a perpendicular direction.
        PieceType::Knight => knight_valid_moves(board, piece, piece_source_index),

        // Pawn move one square forwards, and captures one square diagonally forwards. It can move two squares forward on its first move.
        PieceType::Pawn => pawn_valid_moves(board, piece, piece_source_index),
    }
}
