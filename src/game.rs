use ggez::{Context, graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect, Text}};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, default_board_state::generate_default_board, menu::{clickable::{Clickable, Transform}, menu_state::Menu}, piece::{self, Board, Color::*, Piece, PieceType::*}};
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
    pub(crate) lobby_sync: i32
}

impl Game {
    pub(crate) fn new(ctx: &mut Context, playing_as_white: bool) -> Game {

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
            text: Text::new("Hello I like red"),
            list_item: false
        });

        Game {
            board: generate_default_board(), // Load/create resources such as images here.
            grabbed_piece: None,
            playing_as_white,
            board_mesh: Game::get_board_mesh(ctx),
            active_turn: playing_as_white,
            connection: Networking::new(),
            menu,
            lobby_sync: 0
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

    pub(crate) fn move_piece_index(&mut self, piece_source_index: usize, piece_dest_index: usize) {
        println!(
            "Took: {} {:?}",
            piece_source_index, self.board[piece_source_index]
        );
        let piece = self.board[piece_source_index]
            .take()
            .expect("Error moving piece");
        self.move_piece(piece, piece_dest_index);
    }

    pub(crate) fn move_piece(&mut self, piece: Piece, piece_dest_index: usize) {
        match &self.board[piece_dest_index] {
            Some(Piece{color: White, piece_type: King}) => {
                self.game_over(Black);
            }
            Some(Piece{color: Black, piece_type: King}) => {
                self.game_over(White);
            }
            _ => {}
        }

        // Your turn is over once you've made a move
        self.active_turn = !self.active_turn;
        self.board[piece_dest_index] = Some(piece);
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
