use std::{iter::Zip, ops::RangeInclusive};

use crate::{
    event_handler::BOARD_SIZE,
    render_utilities::{translate_to_coords, translate_to_index},
};
use ggez::graphics::Rect;

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
        PieceType::King => {}
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
        PieceType::Knight => {}
        PieceType::Pawn => {}
    }

    // Returns a list of the valid moves
    indices
}

fn rook_valid_moves(board: &Board, piece: &Piece, piece_source_index: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();

    let (x, y) = translate_to_coords(piece_source_index);

    // for bounds in [right_bound, down_bound, left_bound, up_bound].iter() {
    //     for (x, y) in bounds.clone() {
    //         let dest_index = translate_to_index(x, *y);
    //         println!("Attempting moving to index: {} at ({}, {})", dest_index, x, *y);
    //         if !add_if_can_move(dest_index, piece, &mut indices, board) {
    //             break;
    //         }
    //     }
    // }

    // Check row right
    if x < BOARD_SIZE - 1 {
        for column in (x + 1)..BOARD_SIZE {
            let dest_index = translate_to_index(column, y);
            if !add_if_can_move(dest_index, piece, &mut indices, board) {
                break;
            }
        }
    }

    // Check row left
    if x > 0 {
        for column in (0..x).rev() {
            let dest_index = translate_to_index(column, y);
            if !add_if_can_move(dest_index, piece, &mut indices, board) {
                break;
            }
        }
    }

    // Check column down
    if y < BOARD_SIZE - 1 {
        for row in (y + 1)..BOARD_SIZE {
            let dest_index = translate_to_index(x, row);
            if !add_if_can_move(dest_index, piece, &mut indices, board) {
                break;
            }
        }
    }

    // Check column up
    if y > 0 {
        for row in (0..y).rev() {
            let dest_index = translate_to_index(x, row);
            if !add_if_can_move(dest_index, piece, &mut indices, board) {
                break;
            }
        }
    }

    indices
}

/**
   Adds the given index to the given vector if the move is valid and return true.
   Returns false if the piece cannot move to the index, leaving the vector unchanged.
*/
fn add_if_can_move(
    piece_dest_index: usize,
    piece: &Piece,
    indices: &mut Vec<usize>,
    board: &Board,
) -> bool {
    if let Some(dest_piece) = &board[piece_dest_index] {
        // If the two pieces have equal color, then exclude the move
        if piece.color == dest_piece.color {
            return false;
        } else {
            // If the piece is of opposite color then include it as a move
            // but do not continue after that
            indices.push(piece_dest_index);
            return false;
        }
    } else {
        // If there is no piece then we continue
        indices.push(piece_dest_index);
        return true;
    }
}

fn bishop_valid_moves(board: &Board, piece: &Piece, piece_source_index: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let (x, y) = translate_to_coords(piece_source_index);

    // Up right
    for (x, y) in (x..BOARD_SIZE).zip((0..=y).rev()) {
        let dest_index = translate_to_index(x, y);
        if !add_if_can_move(dest_index, piece, &mut indices, board) {
            break;
        }
    }

    // Down right
    for (x, y) in (x..BOARD_SIZE).zip(y..BOARD_SIZE) {
        let dest_index = translate_to_index(x, y);
        if !add_if_can_move(dest_index, piece, &mut indices, board) {
            break;
        }
    }

    // Up left
    for (x, y) in ((0..=x).rev()).zip((0..=y).rev()) {
        let dest_index = translate_to_index(x, y);
        if !add_if_can_move(dest_index, piece, &mut indices, board) {
            break;
        }
    }

    // Down left
    for (x, y) in ((0..=x).rev()).zip(y..BOARD_SIZE) {
        let dest_index = translate_to_index(x, y);
        if !add_if_can_move(dest_index, piece, &mut indices, board) {
            break;
        }
    }

    indices
}
