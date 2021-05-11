use ggez::{
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
    Context,
};

use crate::{
    default_board_state::generate_default_board,
    piece::{self, Board, Color::*, Piece, PieceType::*},
    render_utilities::{translate_to_coords, translate_to_index},
};
use crate::{event_handler::TILE_SIZE, networking::connection::Networking};

// Main struct
pub(crate) struct Game {
    pub(crate) board: Board,
    pub(crate) grabbed_piece: Option<(Piece, (usize, usize))>,
    pub(crate) playing_as_white: bool,
    pub(crate) board_mesh: Mesh,
    pub(crate) connection: Networking,
    pub(crate) active_turn: bool,
    pub(crate) move_history: Vec<Move>,
}

pub(crate) struct Move {
    pub(crate) piece: Piece,
    pub(crate) piece_source_index: usize,
    pub(crate) piece_dest_index: usize,
}

impl Game {
    pub(crate) fn new(ctx: &mut Context, playing_as_white: bool) -> Game {
        Game {
            board: generate_default_board(), // Load/create resources such as images here.
            grabbed_piece: None,
            playing_as_white,
            board_mesh: Game::get_board_mesh(ctx),
            connection: Networking::new(),
            active_turn: playing_as_white,
            move_history: Vec::new(),
        }
    }

    fn get_board_mesh(ctx: &mut Context) -> Mesh {
        let dark_color: (u8, u8, u8) = (181, 136, 99);
        let light_color: (u8, u8, u8) = (240, 217, 181);

        let mut mesh_builder = MeshBuilder::new();

        let get_rect = |x_index: i32, y_index: i32| {
            return Rect::new_i32(
                x_index * TILE_SIZE,
                y_index * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
            );
        };
        // Calculate sprite batch
        for row in 0..8 {
            for column in 0..8 {
                let color: (u8, u8, u8);
                if (column + row) % 2 == 0 {
                    // White
                    color = light_color;
                } else {
                    color = dark_color;
                }

                // Create Rectangle in mesh at position
                mesh_builder.rectangle(DrawMode::fill(), get_rect(column, row), Color::from(color));
            }
        }
        let mesh = mesh_builder
            .build(ctx)
            .expect("Failed to render game board");
        mesh
    }

    pub(crate) fn move_piece_index(&mut self, piece_source_index: usize, piece_dest_index: usize) {
        println!(
            "Took: {} {:?}",
            piece_source_index, self.board[piece_source_index]
        );
        let piece = self.board[piece_source_index]
            .take()
            .expect("Error moving piece");
        self.move_piece(piece, piece_source_index, piece_dest_index);
    }

    pub(crate) fn move_piece(
        &mut self,
        mut piece: Piece,
        piece_source_index: usize,
        piece_dest_index: usize,
    ) {
        self.move_history.push(Move {
            piece: piece.clone(),
            piece_source_index,
            piece_dest_index,
        });

        // Your turn is over once you've made a move
        self.active_turn = !self.active_turn;

        // Checks if a king is being captured (a player wins)
        match &self.board[piece_dest_index] {
            Some(Piece {
                color: White,
                piece_type: King(_),
            }) => {
                self.game_over(Black);
            }
            Some(Piece {
                color: Black,
                piece_type: King(_),
            }) => {
                self.game_over(White);
            }
            _ => {}
        }

        match &mut piece {
            // If a pawn is moved for the first time its inner value is changed to true, indicating that it has moved and can no longer move two steps in one move.
            Piece {
                piece_type: Pawn(false),
                color: _,
            } => piece.piece_type = Pawn(true),

            Piece {
                piece_type: Pawn(true),
                color,
            } => {
                let (x, y) = translate_to_coords(piece_source_index);
                match color {
                    White => {
                        if (piece_dest_index == translate_to_index(x - 1, y + 1)
                            && self.board[translate_to_index(x - 1, y + 1)].is_none())
                            || (piece_dest_index == translate_to_index(x + 1, y + 1)
                                && self.board[translate_to_index(x + 1, y + 1)].is_none())
                        {
                            self.board[piece_dest_index - 8] = None;
                        }
                    }
                    Black => {
                        if (piece_dest_index == translate_to_index(x - 1, y - 1)
                            && self.board[translate_to_index(x - 1, y - 1)].is_none())
                            || (piece_dest_index == translate_to_index(x + 1, y - 1)
                                && self.board[translate_to_index(x + 1, y - 1)].is_none())
                        {
                            self.board[piece_dest_index + 8] = None;
                        }
                    }
                }
            }

            // If a rook is moved for the first time its inner value is changed to true, indicating that it has moved and cannot be castled with.
            Piece {
                piece_type: Rook(false),
                color: _,
            } => piece.piece_type = Rook(true),

            // Checks if a king is attempting to castle (Hasn't moved before and is attempting to move two or more squares)
            Piece {
                piece_type: King(false),
                color: _,
            } => {
                // Declares that the king has moved
                piece.piece_type = King(true);

                // Indices of possible castling moves (either moving onto the rook or two steps towards it)
                let (rook_kingside, rook_queenside) =
                    (piece_source_index - 3, piece_source_index + 4);
                let (two_steps_kingside, two_steps_queenside) =
                    (piece_source_index - 2, piece_source_index + 2);

                // If attempting to castle kingside
                if piece_dest_index == rook_kingside || piece_dest_index == two_steps_kingside {
                    let rook = self.board[rook_kingside].take().unwrap();
                    self.board[two_steps_kingside] = Some(piece);
                    self.board[piece_source_index - 1] = Some(rook);
                }
                // If attempting to castle queenside
                else if piece_dest_index == two_steps_queenside
                    || piece_dest_index == rook_queenside
                {
                    let rook = self.board[rook_queenside].take().unwrap();
                    self.board[two_steps_queenside] = Some(piece);
                    self.board[piece_source_index + 1] = Some(rook);
                }
                // If moving the king for the first time, but not attempting to castle
                else {
                    self.board[piece_dest_index] = Some(piece)
                }
                // Return since the King has been moved
                return;
            }
            _ => {}
        }

        // Places the piece on the board
        self.board[piece_dest_index] = Some(piece)
    }

    fn game_over(&mut self, winning_color: piece::Color) {
        match winning_color {
            White => {
                println!("Black lost, white won!");
                todo!()
            }
            Black => {
                println!("White lost, black won!");
                todo!()
            }
        }
    }
}
