use crate::piece::Board;
pub(crate) fn flip_board(board: &Board) -> Board {
    let mut flipped_board = board.clone();
    flipped_board.reverse();
	return flipped_board;
}