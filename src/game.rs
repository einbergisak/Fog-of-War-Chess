use ggez::{
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
    Context,
};

use crate::event_handler::TILE_SIZE;
use crate::{
    default_board_state::generate_default_board,
    piece::{Board, Piece},
};

// Main struct
pub(crate) struct Game {
    pub(crate) board: Board,
    pub(crate) grabbed_piece: Option<(Piece, (usize, usize))>,
    pub(crate) flipped_board: bool,
    pub(crate) board_mesh: Mesh,
}

impl Game {
    pub(crate) fn new(ctx: &mut Context) -> Game {
        Game {
            board: generate_default_board(), // Load/create resources such as images here.
            grabbed_piece: None,
            flipped_board: false,
            board_mesh: Game::get_board_mesh(ctx),
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
}
