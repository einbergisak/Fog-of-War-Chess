use crate::{
    event_handler::BOARD_SIZE,
    piece::{Board, Piece},
    render_utilities::{translate_to_coords, translate_to_index},
};

pub(crate) fn rook_valid_moves(
    board: &Board,
    piece: &Piece,
    piece_source_index: usize,
) -> Vec<usize> {
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

pub(crate) fn bishop_valid_moves(
    board: &Board,
    piece: &Piece,
    piece_source_index: usize,
) -> Vec<usize> {
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

pub(crate) fn knight_valid_moves(
    board: &Board,
    piece: &Piece,
    piece_source_index: usize,
) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let (x, y) = translate_to_coords(piece_source_index);

    // Move up then horizontally
    if y > 1 {
        if x > 0 {
            // left
            add_if_can_move(translate_to_index(x - 1, y - 2), piece, &mut indices, board);
        }
        if x < BOARD_SIZE - 1 {
            // right
            add_if_can_move(translate_to_index(x + 1, y - 2), piece, &mut indices, board);
        }
    }

    // Move down then horizontally
    if y < BOARD_SIZE - 2 {
        if x > 0 {
            // left
            add_if_can_move(translate_to_index(x - 1, y + 2), piece, &mut indices, board);
        }
        if x < BOARD_SIZE - 1 {
            add_if_can_move(translate_to_index(x + 1, y + 2), piece, &mut indices, board);
        }
    }

    // Move left then vertically
    if x > 1 {
        if y > 0 {
            // up
            add_if_can_move(translate_to_index(x - 2, y - 1), piece, &mut indices, board);
        }
        if y < BOARD_SIZE - 1 {
            // down
            add_if_can_move(translate_to_index(x - 2, y + 1), piece, &mut indices, board);
        }
    }

    // Move right then vertically
    if x < BOARD_SIZE - 2 {
        if y > 0 {
            // up
            add_if_can_move(translate_to_index(x + 2, y - 1), piece, &mut indices, board);
        }
        if y < BOARD_SIZE - 1 {
            // down
            add_if_can_move(translate_to_index(x + 2, y + 1), piece, &mut indices, board);
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
