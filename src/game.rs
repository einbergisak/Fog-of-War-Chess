use std::time::{Duration, Instant};

use ggez::{
    audio::{SoundSource, Source},
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
    piece::piece::{self, Board, Piece, PieceColor::*, PieceType::*, *},
    time::Time,
};

use crate::{
    menu::{
        clickable::ClickableGroup,
        menu_game_over::{
            GAME_OVER_MENU_HEIGHT, GAME_OVER_MENU_WIDTH, GAME_OVER_START_X, GAME_OVER_START_Y,
        },
    },
    STATE,
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
pub(crate) const ERROR_COLOR: (u8, u8, u8) = (176, 0, 32);

pub(crate) struct Sound {
    pub(crate) movement: Source,
    pub(crate) capture: Source,
    pub(crate) game_end: Source,
}
// Main struct
pub(crate) struct Game {
    pub(crate) board: Board,
    pub(crate) grabbed_piece: Option<Piece>,
    pub(crate) selected_piece: Option<Piece>,
    pub(crate) playing_as_white: bool,
    pub(crate) board_mesh: Mesh,
    pub(crate) active_turn: bool,
    pub(crate) connection: Networking,
    pub(crate) menu: Menu,
    pub(crate) lobby_sync: i32,
    pub(crate) move_history: Vec<Move>,
    pub(crate) promoting_pawn: Option<Move>,
    pub(crate) available_moves: Vec<usize>,
    pub(crate) premove: Option<(Piece, usize)>, // Piece to move and destination index
    pub(crate) winner: Option<PieceColor>,
    pub(crate) is_admin: bool,
    pub(crate) time: Time,
    pub(crate) game_active: bool,
    pub(crate) sound: Sound,
}

impl Game {
    pub(crate) fn new(ctx: &mut Context) -> Game {
        let mut menu = Menu::new(ctx);
        // Create button for main menu
        menu.create_clickables();

        Game {
            board: generate_default_board(), // Load/create resources such as images here.
            grabbed_piece: None,
            selected_piece: None,
            playing_as_white: false,
            board_mesh: Game::get_board_mesh(ctx),
            active_turn: false,
            connection: Networking::new(),
            menu,
            lobby_sync: 0,
            move_history: Vec::new(),
            promoting_pawn: None,
            available_moves: Vec::new(),
            premove: None,
            winner: None,
            is_admin: false,
            time: Time {
                current_time_left: Duration::new(300, 0),
                opponent_time_left: Duration::new(300, 0),
                turn_start: Instant::now(),
                initial_time: Duration::new(300, 0),
                increment: Duration::new(0, 0),
                time_set: false,
            },
            game_active: false,
            sound: Sound {
                movement: ggez::audio::Source::new(ctx, "/move.ogg").unwrap(),
                capture: ggez::audio::Source::new(ctx, "/capture.ogg").unwrap(),
                game_end: ggez::audio::Source::new(ctx, "/game_end.ogg").unwrap(),
            },
        }
    }

    // Start a game and start the clocks
    fn start_game(&mut self) {
        // Cannot start game while in progress
        if self.game_active || self.winner.is_some() {
            return;
        }
        self.time.current_time_left = self.time.initial_time;
        self.time.opponent_time_left = self.time.initial_time;
        self.game_active = true;
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
            self.perform_time_increment();
            // Your turn is over once you've made a move
            self.active_turn = !self.active_turn;
            if !self.game_active {
                self.start_game();
            }
            self.time.turn_start = Instant::now();
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
                color: PieceColor::White,
                piece_type: King(_),
                index: _,
            }) => {
                self.game_over(PieceColor::Black);
            }
            Some(Piece {
                color: PieceColor::Black,
                piece_type: King(_),
                index: _,
            }) => {
                self.game_over(PieceColor::White);
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
                if ((*color == PieceColor::White
                    && (piece_dest_index == translate_to_index(x - 1, y + 1)
                        || piece_dest_index == translate_to_index(x + 1, y + 1)))
                    || (*color == PieceColor::Black
                        && (piece_dest_index == translate_to_index(x - 1, y - 1)
                            || piece_dest_index == translate_to_index(x + 1, y - 1))))
                    && self.board[piece_dest_index].is_none()
                {
                    // Captures the piece behind its destination tile
                    let one_square_back = if let PieceColor::White = piece.color {
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
                    self.update_available_moves();
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
                    println!("Castling queenside");
                    castle(rook_queenside);
                    return;
                }
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
                    available_moves.append(&mut piece::get_valid_move_indices(self, &piece, false));
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

        // Play sound
        if captured_piece.is_some() {
            self.sound
                .capture
                .play()
                .expect("Could not play capture sound");
        } else {
            self.sound
                .movement
                .play()
                .expect("Could not play movement sound");
        }

        if self.active_turn {
            self.connection.send("opponent", &move_.to_string());
            self.grabbed_piece = None;
            self.selected_piece = None;

            println!(
                "Grabbed: {:?}, {:?}",
                self.grabbed_piece, self.selected_piece
            );
        }
        self.perform_time_increment();
        self.active_turn = !self.active_turn;
        if !self.game_active {
            self.start_game();
        }
        self.time.turn_start = Instant::now();
        self.move_history.push(move_);
    }

    pub(crate) fn game_over(&mut self, winning_color: PieceColor) {
        // Cannot game over more than once
        if self.winner.is_some() {
            return;
        }

        self.game_active = false;
        self.grabbed_piece = None;
        self.selected_piece = None;
        STATE
            .get()
            .write()
            .unwrap()
            .event_validation
            .deselect_cursor = true;

        // Play game over sound
        self.sound
            .game_end
            .play()
            .expect("Could not play game over sound");

        match winning_color {
            PieceColor::White => {
                self.winner = Some(PieceColor::White);
                println!("Black lost, white won!");
            }
            PieceColor::Black => {
                self.winner = Some(PieceColor::Black);
                println!("White lost, black won!");
            }
        }

        self.menu.clickables.push(Clickable {
            transform: Transform {
                x: (GAME_OVER_START_X + 100.0) as i32,
                y: (GAME_OVER_START_Y + GAME_OVER_MENU_HEIGHT - 100.0) as i32,
                width: (GAME_OVER_MENU_WIDTH * 0.3) as i32,
                height: (GAME_OVER_MENU_HEIGHT * 0.1) as i32,
            },
            id: String::from("play_again"),
            text: String::from("Play again"),
            list_item: false,
            hovered: false,
            color: Color::from(LIGHT_COLOR),
            group: ClickableGroup::GameOverMenu,
        });

        self.menu.clickables.push(Clickable {
            transform: Transform {
                x: (GAME_OVER_START_X + GAME_OVER_MENU_WIDTH - 100.0 - GAME_OVER_MENU_WIDTH * 0.3)
                    as i32,
                y: (GAME_OVER_START_Y + GAME_OVER_MENU_HEIGHT - 100.0) as i32,
                width: (GAME_OVER_MENU_WIDTH * 0.3) as i32,
                height: (GAME_OVER_MENU_HEIGHT * 0.1) as i32,
            },
            id: String::from("goto_main_menu"),
            text: String::from("Leave"),
            list_item: false,
            hovered: false,
            color: Color::from(LIGHT_COLOR),
            group: ClickableGroup::GameOverMenu,
        });

        // Prevent the current buttons from being cliked next time
        // a button is clicked
        self.menu.clear_clickable_hovers();
    }

    pub(crate) fn reset_game(&mut self) {
        self.board = generate_default_board();
        self.winner = None;
        self.active_turn = false;
        self.grabbed_piece = None;
        self.selected_piece = None;
        self.premove = None;
        self.move_history = Vec::new();
        self.time.turn_start = Instant::now();
        self.promoting_pawn = None;
    }

    #[allow(unused_assignments)]
    pub(crate) fn button_parsing(&mut self, allowed_group: Vec<ClickableGroup>) {
        let read_state = STATE.get().read().unwrap().clone();

        for i in 0..self.menu.clickables.len() {
            if self.menu.clickables[i].hovered
                && allowed_group.contains(&self.menu.clickables[i].group)
            {
                match &self.menu.clickables[i].id[..] {
                    "create_room_button" => {
                        self.connection.send("create_room", "");
                    }
                    "play_again" => {
                        if STATE
                            .get()
                            .read()
                            .unwrap()
                            .event_validation
                            .opponent_name
                            .is_none()
                        {
                            return;
                        }

                        self.reset_game();
                        self.playing_as_white = !self.playing_as_white;
                        self.active_turn = self.playing_as_white;
                        self.time.increment = Duration::from_secs(0);
                        self.time.current_time_left = self.time.initial_time;
                        self.time.opponent_time_left = self.time.initial_time;
                        self.time.turn_start = Instant::now();
                        self.update_available_moves();
                        self.connection.send("play_again", "");
                    }
                    "goto_main_menu" => {
                        STATE.get().write().unwrap().room_id = None;
                        STATE.get().write().unwrap().event_validation.opponent_name = None;
                        self.menu.visible = true;
                        self.reset_game();
                        self.time.time_set = false;
                        self.connection.send("opponent_leave_lobby", "");
                        self.connection.send("list_rooms", "");
                    }
                    "resign_game_button" => {
                        let winner = if self.playing_as_white {
                            PieceColor::Black
                        } else {
                            PieceColor::White
                        };
                        self.game_over(winner);
                        self.connection.send("resign", "");
                    }
                    "submit_name_button" => {
                        if read_state.name.len() > 0 {
                            STATE.get().write().unwrap().entering_name = false;
                            self.connection.send("set_name", &read_state.name);

                            // Delete the button after it has been used
                            let index = self
                                .menu
                                .clickables
                                .iter()
                                .position(|current| {
                                    current.id == String::from("submit_name_button")
                                })
                                .unwrap();
                            self.menu.clickables.remove(index);
                        }
                    }
                    "minute_plus_1" => {
                        self.modify_time(Duration::from_secs(60), true, false);
                    }
                    "minute_plus_5" => {
                        self.modify_time(Duration::from_secs(5 * 60), true, false);
                    }
                    "minute_plus_10" => {
                        self.modify_time(Duration::from_secs(10 * 60), true, false);
                    }
                    "minute_minus_1" => {
                        self.modify_time(Duration::from_secs(60), false, false);
                    }
                    "minute_minus_5" => {
                        self.modify_time(Duration::from_secs(5 * 60), false, false);
                    }
                    "minute_minus_10" => {
                        self.modify_time(Duration::from_secs(10 * 60), false, false);
                    }

                    "second_plus_15" => {
                        self.modify_time(Duration::from_secs(15), true, false);
                    }
                    "second_minus_15" => {
                        self.modify_time(Duration::from_secs(15), false, false);
                    }

                    "increment_plus_1" => {
                        self.modify_time(Duration::from_secs(1), true, true);
                    }
                    "increment_plus_5" => {
                        self.modify_time(Duration::from_secs(5), true, true);
                    }
                    "increment_plus_10" => {
                        self.modify_time(Duration::from_secs(10), true, true);
                    }
                    "increment_minus_1" => {
                        self.modify_time(Duration::from_secs(1), false, true);
                    }
                    "increment_minus_5" => {
                        self.modify_time(Duration::from_secs(5), false, true);
                    }
                    "increment_minus_10" => {
                        self.modify_time(Duration::from_secs(10), false, true);
                    }
                    "finish_time_start_game" => {
                        // Only admin has permission to make changes to the time
                        if self.is_admin {
                            self.time.time_set = true;
                            self.time.current_time_left = self.time.initial_time;
                            self.time.opponent_time_left = self.time.initial_time;

                            if read_state.opponent_online {
                                // If the client is already connected we send the data afterwards
                                self.connection.send(
                                    "set_clock_time",
                                    &format!(
                                        "{}:{}",
                                        self.time.initial_time.as_secs(),
                                        self.time.increment.as_secs()
                                    )[..],
                                );
                            }
                        }
                    }
                    id if self.menu.clickables[i].list_item => {
                        if id.len() != 4 {
                            println!("Wrong formatted id: {}", id);
                        } else {
                            println!("Join room: {}", id);
                            STATE.get().write().unwrap().room_id = Some(String::from(id));
                            self.connection.send("join_room", id);
                        }
                    }
                    data => {
                        println!("Unused button click {}", data);
                    }
                }
                break;
            }
        }
    }

    // Attempt to move a piece
    pub(crate) fn attempt_move(&mut self, piece: Piece, piece_dest_index: usize) {
        let valid_moves = piece::get_valid_move_indices(self, &piece, false);
        println!("Current turn: {}", self.active_turn);
        println!("Valid moves: {:?}", valid_moves);
        if valid_moves.contains(&piece_dest_index) && self.active_turn {
            println!("Move to index {} is valid", piece_dest_index);

            // Promotion
            println!("Moving piece: {:?}", &piece);
            if piece.piece_type == Pawn(true)
                && ((piece.color == White
                    && translate_to_coords(piece_dest_index).1 == BOARD_SIZE - 1)
                    || (piece.color == Black && translate_to_coords(piece_dest_index).1 == 0))
            {
                println!("Noticed pawn promotion");
                self.promoting_pawn = Some(Move {
                    piece,
                    piece_dest_index,
                    captured_piece: None, // It is assigned an eventual captured piece when the promotion has been confirmed (mouse button down event)
                    move_type: Promotion(King(true)), // Default invalid value that is later changed when the player has selected which piece to promote into.
                });
                return;
            }

            self.move_grabbed_piece(piece, piece_dest_index);
        }
        // If not your turn, add the move as a premove (if there isn't already one)
        else if !self.active_turn && self.premove.is_none() {
            self.board[piece.index] = Some(piece);

            // TODO: Premove constraints
            if piece_dest_index != piece.index
                && get_valid_move_indices(self, &piece, true).contains(&piece_dest_index)
            {
                println!(
                    "It's not your turn. Adding premove to index {} ",
                    piece_dest_index
                );
                self.premove = Some((piece, piece_dest_index));
            }
        } else {
            println!("Move to index {} is NOT valid.", piece_dest_index);
            // // Reset position to source
            self.board[piece.index] = Some(piece);
        }
    }
}
