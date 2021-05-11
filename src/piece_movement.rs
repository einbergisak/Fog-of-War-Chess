use crate::{
    event_handler::BOARD_SIZE,
    game::Move,
    piece::{Board, Color::*, Piece, PieceType::*},
    render_utilities::translate_to_index,
};

// Kan optimeras (slippa repetitiv kod) med macro men jag fattar inte sånt
pub(crate) fn rook_valid_moves(board: &Board, piece: &Piece) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();

    let (x, y) = piece.get_pos();

    // Check row x+
    if x < BOARD_SIZE - 1 {
        for column in (x + 1)..BOARD_SIZE {
            let dest_index = translate_to_index(column, y);
            if !add_if_can_move(dest_index, piece, &mut indices, board) {
                break;
            }
        }
    }

    // Check row x-
    if x > 0 {
        for column in (0..x).rev() {
            let dest_index = translate_to_index(column, y);
            if !add_if_can_move(dest_index, piece, &mut indices, board) {
                break;
            }
        }
    }

    // Check column y+
    if y < BOARD_SIZE - 1 {
        for row in (y + 1)..BOARD_SIZE {
            let dest_index = translate_to_index(x, row);
            if !add_if_can_move(dest_index, piece, &mut indices, board) {
                break;
            }
        }
    }

    // Check column y-
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

// Kan optimeras (slippa repetitiv kod) med macro men jag fattar inte sånt
pub(crate) fn bishop_valid_moves(board: &Board, piece: &Piece) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let (x, y) = piece.get_pos();

    // Towards x+ y+
    for (dest_x, dest_y) in (x..BOARD_SIZE).zip(y..BOARD_SIZE) {
        let dest_index = translate_to_index(dest_x, dest_y);
        if !(x == dest_x && y == dest_y) && !add_if_can_move(dest_index, piece, &mut indices, board)
        {
            break;
        }
    }

    // Towards x+ y-
    for (dest_x, dest_y) in (x..BOARD_SIZE).zip((0..=y).rev()) {
        let dest_index = translate_to_index(dest_x, dest_y);
        if !(x == dest_x && y == dest_y) && !add_if_can_move(dest_index, piece, &mut indices, board)
        {
            break;
        }
    }

    // Towards x- y-
    for (dest_x, dest_y) in ((0..=x).rev()).zip((0..=y).rev()) {
        let dest_index = translate_to_index(dest_x, dest_y);
        if !(x == dest_x && y == dest_y) && !add_if_can_move(dest_index, piece, &mut indices, board)
        {
            break;
        }
    }

    // Towards x- y+
    for (dest_x, dest_y) in ((0..=x).rev()).zip(y..BOARD_SIZE) {
        let dest_index = translate_to_index(dest_x, dest_y);
        if !(x == dest_x && y == dest_y) && !add_if_can_move(dest_index, piece, &mut indices, board)
        {
            break;
        }
    }

    indices
}

pub(crate) fn knight_valid_moves(board: &Board, piece: &Piece) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let (x, y) = piece.get_pos();

    // Move 2 steps towards y- then 1 step horizontally
    if y > 1 {
        if x > 0 {
            // second step towards x-
            add_if_can_move(translate_to_index(x - 1, y - 2), piece, &mut indices, board);
        }
        if x < BOARD_SIZE - 1 {
            // second step towards x+
            add_if_can_move(translate_to_index(x + 1, y - 2), piece, &mut indices, board);
        }
    }

    // Move 2 steps towards y+ then 1 step horizontally
    if y < BOARD_SIZE - 2 {
        if x > 0 {
            // second step towards x-
            add_if_can_move(translate_to_index(x - 1, y + 2), piece, &mut indices, board);
        }
        if x < BOARD_SIZE - 1 {
            // second step towards x+
            add_if_can_move(translate_to_index(x + 1, y + 2), piece, &mut indices, board);
        }
    }

    // Move 2 steps towards x- then 1 step vertically
    if x > 1 {
        if y > 0 {
            // second step towards y-
            add_if_can_move(translate_to_index(x - 2, y - 1), piece, &mut indices, board);
        }
        if y < BOARD_SIZE - 1 {
            // second step towards y+
            add_if_can_move(translate_to_index(x - 2, y + 1), piece, &mut indices, board);
        }
    }

    // Move 2 steps towards x+ then 1 step vertically
    if x < BOARD_SIZE - 2 {
        if y > 0 {
            // second step towards y-
            add_if_can_move(translate_to_index(x + 2, y - 1), piece, &mut indices, board);
        }
        if y < BOARD_SIZE - 1 {
            // second step towards y+
            add_if_can_move(translate_to_index(x + 2, y + 1), piece, &mut indices, board);
        }
    }

    indices
}

pub(crate) fn king_valid_moves(board: &Board, piece: &Piece) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let (x, y) = piece.get_pos();

    for xd in -1..=1 {
        for yd in -1..=1 {
            // Makes sure that it doesn't go outside of the board index bounds
            let (dest_x, dest_y) = (x as i32 + xd, y as i32 + yd);
            if dest_x >= 0
                && dest_x < BOARD_SIZE as i32
                && dest_y >= 0
                && dest_y < BOARD_SIZE as i32
                && !(dest_x == x as i32 && dest_y == y as i32)
            {
                add_if_can_move(
                    translate_to_index(dest_x as usize, dest_y as usize),
                    piece,
                    &mut indices,
                    board,
                );
            }
        }
    }

    // Castling: The King can castle with a friendly rook so long as neither of the pieces have moved.
    if let Piece {
        piece_type: King(false), // King(false) is a king that hasn't moved.
        color: king_color,
        index: _,
    } = piece
    {
        let piece_source_index = piece.get_index();
        'rook_loop: for rook_index in [piece_source_index - 3, piece_source_index + 4].iter() {
            if let Some(Piece {
                piece_type: Rook(false), // Rook(false) is a rook that hasn't moved
                color: rook_color,
                index: _,
            }) = board[*rook_index].as_ref()
            {
                if rook_color == king_color {
                    if *rook_index < piece_source_index {
                        // Check for pieces between the king and the kingside rook
                        for index in (*rook_index + 1)..piece_source_index {
                            if board[index].is_some() {
                                // If there is a piece between the king and the kingside rook we continue to check the kingside rook
                                continue 'rook_loop;
                            }
                        }
                        indices.push(piece_source_index - 2); // Lets the player castle by moving two squares
                        indices.push(*rook_index); // Lets the player castle by moving to the rook
                    } else {
                        // Check for pieces between the king and the queenside rook
                        for index in (piece_source_index + 1)..*rook_index {
                            if board[index].is_some() {
                                // If there is a piece between the king and the queenside rook
                                break 'rook_loop;
                            }
                        }
                        indices.push(piece_source_index + 2); // Lets the player castle by moving two squares
                        indices.push(*rook_index); // Lets the player castle by moving to the rook
                    }
                }
            }
        }
    }

    indices
}

pub(crate) fn pawn_valid_moves(
    board: &Board,
    piece: &Piece,
    move_history: &Vec<Move>,
) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    let (x, y) = piece.get_pos();

    let y_direction: i32 = if let White = piece.color {
        // White pawn moves in positive y direction
        1
    } else {
        // Black pawn moves in negative y direction
        -1
    };

    let one_forwards = (y as i32 + 1 * y_direction) as usize;
    let two_forwards = (y as i32 + 2 * y_direction) as usize;

    // If there is no piece blocking the pawn
    if board[translate_to_index(x, one_forwards)].is_none() {
        // Lets the pawn move two spaces forwards on its first move
        if piece.piece_type == Pawn(false) && board[translate_to_index(x, two_forwards)].is_none() {
            indices.push(translate_to_index(x, two_forwards))
        }
        // Regular move 1 square forwards
        indices.push(translate_to_index(x, one_forwards));
    }

    let mut pawn_capture = |x_direction: i32| {
        let adjacent_x = (x as i32 + x_direction) as usize;

        // Regular diagonal capture
        if let Some(Piece {
            piece_type: _,
            color: other_color,
            index: _,
        }) = board[translate_to_index(adjacent_x, one_forwards)].as_ref()
        {
            if &piece.color != other_color {
                indices.push(translate_to_index(adjacent_x, one_forwards))
            }
        }

        // En passant capture
        if (piece.color == White && y == 4) || (piece.color == Black && y == 3) {
            println!("Last move: {:?}", move_history.last());
            if let Some(Move {
                piece:
                    Piece {
                        piece_type: Pawn(_),
                        color: other_color,
                        index: other_pawn_previous_index,
                    },
                piece_dest_index: other_pawn_current_index,
                captured_piece: _,
                move_type: _,
            }) = move_history.last()
            {
                if *other_pawn_previous_index == translate_to_index(adjacent_x, two_forwards)
                    && *other_pawn_current_index == translate_to_index(adjacent_x, y)
                    && &piece.color != other_color
                {
                    println!("Yeet 2");
                    indices.push(translate_to_index(adjacent_x, one_forwards))
                }
                println!("Yeet 3");
            }
        }
    };

    // If the pawn is not on the edge of the board (kingside)
    if x > 0 {
        // Pawn capture, diagonally in negative x direction (kingside)
        pawn_capture(-1)
    }

    // If the pawn is not on the edge of the board (queenside)
    if x < BOARD_SIZE - 1 {
        // Pawn capture, diagonally in positive x direction (queenside)
        pawn_capture(1)
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
