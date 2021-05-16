use std::iter::FromIterator;

use self::MoveType::*;
use crate::piece::piece::{
    self, Piece,
    PieceType::{self, *},
};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Move {
    pub(crate) piece: Piece,
    pub(crate) piece_dest_index: usize,
    pub(crate) captured_piece: Option<Piece>,
    pub(crate) move_type: MoveType,
}

impl ToString for Move {
    fn to_string(&self) -> String {
        let (captured_type, captured_color, captured_index) = if let Some(p) = &self.captured_piece
        {
            (
                p.piece_type.to_str(),
                p.color.to_str(),
                p.get_index() as i32,
            )
        } else {
            ("-", "", -1)
        };
        return format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            self.piece.piece_type.to_str(),
            self.piece.color.to_str(),
            self.piece.index,
            self.piece_dest_index,
            captured_type,
            captured_color,
            captured_index,
            self.move_type.to_str()
        );
    }
}

impl Move {
    pub(crate) fn from_str(string: String) -> Self {
        // Removes prefix and suffix
        let mut chars = string.chars();
        chars.next();
        chars.next_back();

        let s: String = String::from_iter(chars);
        let mut s = s.split(":");

        let piece_type = PieceType::from_str(s.next().unwrap());
        let color = piece::PieceColor::from_str(s.next().unwrap());
        let index = s.next().unwrap().parse::<usize>().unwrap();
        let piece = Piece {
            piece_type,
            color,
            index,
        };

        let piece_dest_index = s.next().unwrap().parse::<usize>().unwrap();

        let captured_piece = {
            let captured_type = s.next().unwrap();
            if captured_type == "-" {
                // Skips the captured color and index since there is no captured piece
                s.next();
                s.next();
                None
            } else {
                let captured_type = PieceType::from_str(captured_type);
                let captured_color = piece::PieceColor::from_str(s.next().unwrap());
                let captured_index = s.next().unwrap().parse::<usize>().unwrap();
                let captured_piece = Piece {
                    piece_type: captured_type,
                    color: captured_color,
                    index: captured_index,
                };
                Some(captured_piece)
            }
        };
        let move_type = MoveType::from_str(s.next().unwrap());
        Move {
            piece,
            piece_dest_index,
            captured_piece,
            move_type,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum MoveType {
    Regular,
    EnPassant,
    Promotion(PieceType), // Inner value is the piece type you're promoting to.
    Castle,
}

impl MoveType {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            Regular => "r",
            EnPassant => "ep",
            Promotion(piece_type) => match piece_type {
                Queen => "pq",
                Rook(_) => "pr",
                Bishop => "pb",
                Knight => "pn",
                _ => panic!("Invalid promotion type"),
            },
            Castle => "c",
        }
    }

    pub(crate) fn from_str(string: &str) -> Self {
        let mut chars = string.chars();

        // If promoting
        if chars.next().unwrap() == 'p' {
            Promotion(match chars.next().unwrap() {
                'q' => Queen,
                'r' => Rook(true),
                'b' => Bishop,
                'n' => Knight,
                _ => panic!("Invalid promotion type"),
            })
        } else {
            match string {
                "r" => Regular,
                "ep" => EnPassant,
                "c" => Castle,
                _ => panic!("Invalid input when attempting to convert &str to MoveType"),
            }
        }
    }
}
