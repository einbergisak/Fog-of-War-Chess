use crate::{event_handler::BOARD_SIZE, render_utilities::{translate_to_coords, translate_to_index}};
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
    pub(crate) color: Color
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

pub(crate) fn is_valid_move(move_index: usize, valid_moves: Vec<usize>) -> bool {
    return valid_moves.contains(&move_index);
}

pub(crate) fn get_valid_move_indices(board: &Board, piece_index: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    println!("Piece index {}", piece_index);
    println!("yeet2 {:?}", board[piece_index]);
    if let Some(Piece{piece_type, color}) = &board[piece_index] {
        println!("yeet3 {:?}", board[piece_index]);

        match piece_type {
            PieceType::King => {}
            PieceType::Queen => { // Queen moves as both Rook and Bishop
                indices.append(&mut rook_movement(board, piece_index));
                indices.append(&mut bishop_movement(board, piece_index));
            }
            PieceType::Rook => { // Horizontal + Vertical movement
                indices.append(&mut rook_movement(board, piece_index));
            }
            PieceType::Bishop => { // Diagonal movement
                indices.append(&mut bishop_movement(board, piece_index));
            }
            PieceType::Knight => {}
            PieceType::Pawn => {}
        }
    }

    indices
}

fn rook_movement(board: &Board, piece_index: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();

    let (x, y) = translate_to_coords(piece_index);
    let ys = [y, y, y, y, y, y, y, y];
    let xs = [x, x, x, x, x, x, x, x];
    let right_bound = (x..=BOARD_SIZE-1).zip(ys.iter());
    let left_bound = (x - 1..=0).zip(ys.iter());
    let down_bound = (y..=BOARD_SIZE-1).zip(xs.iter());
    let up_bound = (y.saturating_sub(1)..=0).zip(xs.iter());

    println!("HELLO from the OTHER SIDE");

    for bounds in [
        right_bound,
        down_bound,
        left_bound,
        up_bound,
    ].iter() {
        for (x, y) in bounds.clone() {
            let index = translate_to_index(x, *y);
            println!("{} {} {}", index, x, *y);
            if !add_if_can_move(index, piece_index, &mut indices, board) {
                break;
                print!("YEEEEEEEEEEEEEEEEEEEEEEEEET");
            }
        }
    }

    // Check row right
    /* for column in  {
        let index = translate_to_index(column, y);
        if !add_if_can_move(index, piece_index, &mut indices, board) {
            break;
        }
    }

    // Check row left
    for column in x - 1..=0 {
        let index = translate_to_index(column, y);
        if !add_if_can_move(index, piece_index, &mut indices, board) {
            break;
        }
    }

    // Check column down
    for row in y..BOARD_SIZE {
        let index = translate_to_index(x, row);
        if !add_if_can_move(index, piece_index, &mut indices, board) {
            break;
        }
    }

    // Check column up
    for row in y - 1..=0 {
        let index = translate_to_index(x, row);
        if !add_if_can_move(index, piece_index, &mut indices, board) {
            break;
        }
    } */

    indices
}

/**
   Adds the given index to the given vector if the move is valid and return true.
   Returns false if the piece cannot move to the index, leaving the vector unchanged.
*/
fn add_if_can_move(
    index: usize,
    initial_piece_index: usize,
    indices: &mut Vec<usize>,
    board: &Board,
) -> bool {
    if let Some(piece) = &board[index] {
        // If the two pieces have equal color, then exclude the move
        if piece.color == board[initial_piece_index].as_ref().unwrap().color {
            return false;
        } else {
            // If the piece is of opposite color then include it as a move
            // but do not continue after that
            indices.push(index);
            return false;
        }
    } else {
        // If there is no piece then we continue
        indices.push(index);
        return true;
    }
}

fn bishop_movement(board: &Board, piece_index: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();

    let (x, y) = translate_to_coords(piece_index);

    let up_right_bounds = (x..=(BOARD_SIZE-1)).zip(y..=0);
    let down_right_bounds = (x..=(BOARD_SIZE-1)).zip(y..=(BOARD_SIZE-1));
    let up_left_bounds = (x..=0).zip(y..=0);
    let down_left_bounds = (x..=0).zip(y..=(BOARD_SIZE-1));

    for bounds in [
        up_right_bounds,
        down_right_bounds,
        up_left_bounds,
        down_left_bounds,
    ].iter() {
        for (x, y) in bounds.clone() {
            let index = translate_to_index(x, y);
            if !add_if_can_move(index, piece_index, &mut indices, board) {
                break;
            }
        }
    }

    indices
}
