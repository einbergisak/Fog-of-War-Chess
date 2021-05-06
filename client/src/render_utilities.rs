use crate::{event_handler::BOARD_SIZE, piece::Board};

pub(crate) fn flip_board(board: &Board) -> Board {
    let mut flipped_board = board.clone();
    flipped_board.reverse();
    return flipped_board;
}

pub(crate) fn flip_index(index: &i32, board_size: i32) -> i32 {
    let list_size = board_size * board_size;
    return list_size - index - 1;
}

/// Translates from game coordinates to list index
pub(crate) fn translate_to_index(x: usize, y: usize) -> usize {
    return y * BOARD_SIZE + x;
}

/// Translates from list index to game coordinates
pub(crate) fn translate_to_coords(index: usize) -> (usize, usize) {
    let y = index / 8;
    let x = index % 8;
    return (x, y);
}