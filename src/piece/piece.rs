use ggez::graphics::Rect;

use crate::{
    game::Game,
    piece::{
        piece::PieceType::*,
        piece_movement::{
            bishop_valid_moves, king_valid_moves, knight_valid_moves, pawn_valid_moves,
            rook_valid_moves,
        },
    },
    render_utilities::translate_to_coords,
};

pub(crate) type Board = Vec<Option<Piece>>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            PieceColor::White => "w",
            PieceColor::Black => "b",
        }
    }
    pub(crate) fn from_str(string: &str) -> Self {
        match string {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => panic!("Error converting &str to Color"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum PieceType {
    King(bool), // Inner boolean which is true if the king has moved (used for castling)
    Queen,
    Rook(bool), // Inner boolean which is true if the rook has moved (used for castling)
    Bishop,
    Knight,
    Pawn(bool), // Inner boolean which is true if the pawn has moved (pawns can move two steps on their first move)
}

impl PieceType {
    /// Returns corresponding &str
    pub(crate) fn to_str(&self) -> &str {
        // The 't' means that its inner value is true, which is the case for all moved pieces.
        match self {
            King(true) => "Kt",
            King(false) => "Kf",
            Queen => "Q",
            Rook(true) => "Rt",
            Rook(false) => "Rf",
            Bishop => "B",
            Knight => "N",
            Pawn(true) => "Pt",
            Pawn(false) => "Pf",
        }
    }
    /// Returns corresponding PieceType
    pub(crate) fn from_str(string: &str) -> Self {
        // The 't' means that its inner value is true, which is the case for all moved pieces.
        match string {
            "Kt" => King(true),
            "Kf" => King(false),
            "Q" => Queen,
            "Rt" => Rook(true),
            "Rf" => Rook(false),
            "B" => Bishop,
            "N" => Knight,
            "Pt" => Pawn(true),
            "Pf" => Pawn(false),
            _ => panic!("Recieved invalid string when converting to PieceType"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: PieceColor,
    pub(crate) index: usize,
}

impl Piece {
    pub(crate) fn get_pos(&self) -> (usize, usize) {
        translate_to_coords(self.get_index())
    }
    pub(crate) fn get_index(&self) -> usize {
        self.index.clone()
    }
}

pub(crate) fn get_piece_rect(piece: &Piece) -> Rect {
    let src_image_y = match piece.color {
        PieceColor::White => 0.0,
        PieceColor::Black => 0.5,
    };
    let src_image_x = match piece.piece_type {
        PieceType::King(_) => 0.0,
        PieceType::Queen => 1.0 / 6.0,
        PieceType::Bishop => 2.0 / 6.0,
        PieceType::Knight => 3.0 / 6.0,
        PieceType::Rook(_) => 4.0 / 6.0,
        PieceType::Pawn(_) => 5.0 / 6.0,
    };

    Rect::new(src_image_x, src_image_y, 1.0 / 6.0, 0.5)
}

/// Returns the indices of the valid moves that the piece can make.
/// If the move is a premove, it will return all indices in range with no regard to if there are pieces blocking or if certain conditions are met.
pub(crate) fn get_valid_move_indices(game: &Game, piece: &Piece, is_premove: bool) -> Vec<usize> {
    let board = &game.board;

    // Returns a list of the valid moves
    match piece.piece_type {
        // King moves one square in any direction
        PieceType::King(_) => king_valid_moves(board, piece, is_premove),

        // Queen moves diagonally, vertically or horizontally (Rook + Bishop)
        PieceType::Queen => {
            let mut moves = bishop_valid_moves(board, piece, is_premove);
            moves.append(&mut rook_valid_moves(board, piece, is_premove));
            moves
        }

        // Rook moves vertically or horizontally
        PieceType::Rook(_) => rook_valid_moves(board, piece, is_premove),

        // Bishop moves diagonally
        PieceType::Bishop => bishop_valid_moves(board, piece, is_premove),

        // Knight moves two steps either vertically or horizontally, then one step in a perpendicular direction.
        PieceType::Knight => knight_valid_moves(board, piece, is_premove),

        // Pawn move one square forwards, and captures one square diagonally forwards. It can move two squares forward on its first move.
        PieceType::Pawn(_) => pawn_valid_moves(board, piece, &game.move_history, is_premove),
    }
}
