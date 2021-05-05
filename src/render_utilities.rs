use crate::piece::Board;
pub(crate) fn flip_board(board: &Board) -> Board {
    let mut flipped_board = board.clone();
    flipped_board.reverse();
    return flipped_board;
}

pub(crate) fn flip_index(index: &i32, board_size: i32) -> i32 {
    let list_size = board_size * board_size;
    return list_size - index - 1;
}
