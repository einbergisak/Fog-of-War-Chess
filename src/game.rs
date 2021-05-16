use ggez::{
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
    Context,
};

use crate::{
    default_board_state::generate_default_board,
    menu::{
        clickable::{Clickable, Transform},
        menu_state::Menu,
    },
    move_struct::MoveType,
    piece::piece::{self, Board, Piece, PieceColor::*, PieceType::*},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

use crate::{
    event_handler::BOARD_SIZE,
    move_struct::{Move, MoveType::*},
    render_utilities::{translate_to_coords, translate_to_index},
};

use crate::{event_handler::TILE_SIZE, networking::connection::Networking};

pub(crate) const BACKGROUND_COLOR: (u8, u8, u8) = (57, 43, 20);
pub(crate) const DARK_COLOR: (u8, u8, u8) = (181, 136, 99);
pub(crate) const LIGHT_COLOR: (u8, u8, u8) = (240, 217, 181);

// Main struct
pub(crate) struct Game {
    pub(crate) board: Board,
    pub(crate) grabbed_piece: Option<(Piece, (usize, usize))>,
    pub(crate) playing_as_white: bool,
    pub(crate) board_mesh: Mesh,
    pub(crate) active_turn: bool,
    pub(crate) connection: Networking,
    pub(crate) menu: Menu,
    pub(crate) lobby_sync: i32,
    pub(crate) move_history: Vec<Move>,
    pub(crate) promoting_pawn: Option<Move>,
    pub(crate) available_moves: Vec<usize>,
}

impl Game {
    pub(crate) fn new(ctx: &mut Context) -> Game {
        let mut menu = Menu::new();
        menu.clickables.push(Clickable {
            id: String::from("create_room_button"),
            transform: Transform {
                x: SCREEN_WIDTH as i32 / 4 - 500 / 2,
                y: SCREEN_HEIGHT as i32 / 2 - 200 / 2,
                width: 500,
                height: 200,
            },
            color: Color::from(LIGHT_COLOR),
            hovered: false,
            text: String::from("Create room"),
            list_item: false,
        });

        Game {
            board: generate_default_board(), // Load/create resources such as images here.
            grabbed_piece: None,
            playing_as_white: false,
            board_mesh: Game::get_board_mesh(ctx),
            active_turn: false,
            connection: Networking::new(),
            menu,
            lobby_sync: 0,
            move_history: Vec::new(),
            promoting_pawn: None,
            available_moves: Vec::new(),
        }
    }

    fn get_board_mesh(ctx: &mut Context) -> Mesh {
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
                    color = LIGHT_COLOR;
                } else {
                    color = DARK_COLOR;
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

    pub(crate) fn move_piece_from_board(&mut self, move_: Move) {
        let piece_source_index = move_.piece.index;
        let piece_dest_index = move_.piece_dest_index;
        println!(
            "Took: {} {:?}",
            piece_source_index, self.board[piece_source_index]
        );
        let piece = self.board[piece_source_index]
            .take()
            .expect("Error moving piece");

        // Promotion is different from other moves, since you also need the promoting type, not just the source and destination index.
        if let Promotion(piece_type) = move_.move_type {
            println!("Noticed promotion!!!!");
            let captured_piece = self.board[piece_dest_index].take();
            self.board[piece_dest_index] = Some(Piece {
                piece_type,
                color: piece.color,
                index: piece_dest_index,
            });
            self.move_history.push(Move {
                piece: piece,
                piece_dest_index: piece_dest_index,
                captured_piece,
                move_type: Promotion(piece_type),
            });
            // Your turn is over once you've made a move
            self.active_turn = !self.active_turn;
            self.update_available_moves();
        } else {
            println!("Moving! active_turn: {}", self.active_turn);
            self.move_grabbed_piece(piece, move_.piece_dest_index);
        }
    }

    pub(crate) fn move_grabbed_piece(&mut self, mut piece: Piece, piece_dest_index: usize) {
        // Checks if a king is being captured (a player wins)
        match &self.board[piece_dest_index] {
            Some(Piece {
                color: White,
                piece_type: King(_),
                index: _,
            }) => {
                self.game_over(Black);
            }
            Some(Piece {
                color: Black,
                piece_type: King(_),
                index: _,
            }) => {
                self.game_over(White);
            }
            _ => {}
        }

        match &piece {
            // If a pawn is moved for the first time its inner value is changed to true, indicating that it has moved and can no longer move two steps in one move.
            Piece {
                piece_type: Pawn(false),
                color: _,
                index: _,
            } => piece.piece_type = Pawn(true),

            // En passant
            Piece {
                piece_type: Pawn(true),
                color,
                index: _,
            } => {
                let (x, y) = piece.get_pos();

                // If a pawn is to move diagonally without capturing, it must be attempting en passant
                if ((*color == White
                    && (piece_dest_index == translate_to_index(x - 1, y + 1)
                        || piece_dest_index == translate_to_index(x + 1, y + 1)))
                    || (*color == Black
                        && (piece_dest_index == translate_to_index(x - 1, y - 1)
                            || piece_dest_index == translate_to_index(x + 1, y - 1))))
                    && self.board[piece_dest_index].is_none()
                {
                    // Captures the piece behind its destination tile
                    let one_square_back = if let White = piece.color {
                        piece_dest_index - BOARD_SIZE
                    } else {
                        piece_dest_index + BOARD_SIZE
                    };
                    self.move_to_end_turn(
                        &mut piece,
                        piece_dest_index,
                        Some(one_square_back),
                        EnPassant,
                    );
                    piece.index = piece_dest_index;
                    self.board[piece_dest_index] = Some(piece);
                    self.update_available_moves();
                    return;
                }
            }

            // If a rook is moved for the first time its inner value is changed to true, indicating that it has moved and cannot be castled with.
            Piece {
                piece_type: Rook(false),
                color: _,
                index: _,
            } => piece.piece_type = Rook(true),

            // Checks if a king is attempting to castle (Hasn't moved before and is attempting to move two or more squares)
            Piece {
                piece_type: King(false),
                color: _,
                index: _,
            } => {
                // Declares that the king has moved
                piece.piece_type = King(true);
                let piece_source_index = piece.get_index();
                // Indices of possible castling moves (either moving onto the rook or two steps towards it)
                let (rook_kingside, rook_queenside) =
                    (piece_source_index - 3, piece_source_index + 4);
                let (two_steps_kingside, two_steps_queenside) =
                    (piece_source_index - 2, piece_source_index + 2);

                let mut castle = |rook_pos: usize| {
                    let (two_steps_towards_rook, direction) =
                        if translate_to_coords(rook_pos).0 == 0 {
                            (two_steps_kingside, -1)
                        } else {
                            (two_steps_queenside, 1)
                        };
                    let mut rook = self.board[rook_pos].take().unwrap();
                    self.move_to_end_turn(&mut piece, piece_dest_index, None, Castle);
                    let mut piece = piece.clone();
                    piece.index = two_steps_towards_rook;
                    rook.index = (piece_source_index as i32 + direction) as usize;
                    self.board[two_steps_towards_rook] = Some(piece);
                    self.board[(piece_source_index as i32 + direction) as usize] = Some(rook);
                };

                // If attempting to castle kingside
                if piece_dest_index == rook_kingside || piece_dest_index == two_steps_kingside {
                    println!("Castling kingside");
                    castle(rook_kingside);
                    return;
                }
                // If attempting to castle queenside
                else if piece_dest_index == two_steps_queenside
                    || piece_dest_index == rook_queenside
                {
                    castle(rook_queenside);
                    return;
                }
                self.update_available_moves();
            }
            _ => {}
        }

        // Matching special moves. Although this match statement has the same &piece as the one above,
        // this one is needed to let the above moves fall under the "_ => {}" match arm.
        self.move_to_end_turn(
            &mut piece,
            piece_dest_index,
            Some(piece_dest_index),
            Regular,
        );
        piece.index = piece_dest_index;
        self.board[piece_dest_index] = Some(piece);

        self.update_available_moves();
    }

    /// Updates self.available_moves, called at the end of every turn
    pub(crate) fn update_available_moves(&mut self) {
        let mut available_moves: Vec<usize> = Vec::new();
        for tile in &self.board {
            if let Some(piece) = tile {
                if (piece.color == White && self.playing_as_white)
                    || (piece.color == Black && !self.playing_as_white)
                {
                    available_moves.push(piece.index);
                    available_moves.append(&mut piece::get_valid_move_indices(self, &piece));
                }
            }
        }
        self.available_moves = available_moves;
    }

    fn move_to_end_turn(
        &mut self,
        piece: &mut Piece,
        piece_dest_index: usize,
        capture_index: Option<usize>,
        move_type: MoveType,
    ) {
        let captured_piece = if let Some(index) = capture_index {
            self.board[index].take()
        } else {
            None
        };
        let move_ = Move {
            piece: piece.clone(),
            piece_dest_index,
            captured_piece,
            move_type,
        };
        if self.active_turn {
            self.connection.send("opponent", &move_.to_string());
        }
        self.active_turn = !self.active_turn;
        self.move_history.push(move_);
    }

    fn game_over(&mut self, winning_color: piece::PieceColor) {
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

    pub(crate) fn button_parsing(&mut self) {
        for i in 0..self.menu.clickables.len() {
            if self.menu.clickables[i].hovered {
                match &self.menu.clickables[i].id[..] {
                    "create_room_button" => {
                        self.connection.send("create_room", "");
                    }
                    id => {
                        println!("Join room: {}", id);
                        self.connection.send("join_room", id)
                    }
                }
            }
        }
    }
}
