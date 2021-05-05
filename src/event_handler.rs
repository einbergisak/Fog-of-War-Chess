use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{
        self, spritebatch::SpriteBatch, Color, DrawMode, DrawParam, Image, MeshBuilder, Rect,
    },
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{Game, piece::get_piece_rect, render_utilities::{flip_board, flip_index}};

const BOARD_SIZE: usize = 8;
const TILE_SIZE: i32 = 100;
const BOARD_WIDTH: i32 = BOARD_SIZE as i32 * TILE_SIZE;

// Translates from coordinates to list index
fn translate(x: usize, y: usize) -> usize {
    return (y - 1) * BOARD_SIZE + x - 1;
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {


        }
        Ok(())
    }
    // y * 8 + x
    // (y-1) * 8 + x - 1
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
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
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, (Point2::<f32>::new(0.0, 0.0),))?;

        let piece_image = Image::new(ctx, "/pieces.png")?;
        let mut piece_batch = SpriteBatch::new(piece_image);

        let render_board = if self.flipped_board {
            flip_board(&self.board)
        } else {
            self.board.clone()
        };

        for (index, tile) in render_board.iter().enumerate() {
            match tile {
                Some(piece) => {
                    let rect = get_piece_rect(piece);

                    let y = index / BOARD_SIZE;
                    let x = index % BOARD_SIZE;
                    let param = DrawParam::default().src(rect).dest(Point2::new(
                        (x as f32) * TILE_SIZE as f32,
                        (y as f32) * TILE_SIZE as f32,
                    ));

                    piece_batch.add(param);
                }
                None => {}
            }
        }

		if let Some((piece, _position)) = &self.grabbed_piece {
            let rect = get_piece_rect(&piece);
            let (mut x, mut y) = (ggez::input::mouse::position(ctx).x, ggez::input::mouse::position(ctx).y);
			let param = DrawParam::default().src(rect).dest(Point2::new(x - TILE_SIZE as f32 / 2.0, y - TILE_SIZE as f32 / 2.0));
			piece_batch.add(param);
        };

        graphics::draw(ctx, &piece_batch, (Point2::<f32>::new(0.0, 0.0),))?;

        // Draw pieces
        //let mut piece_batch = SpriteBatch::new()

        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                let (start_x, start_y) = (0.0, 0.0);

                // Out of bounds checking
                if start_x + x > BOARD_WIDTH as f32
                    || start_y + y > BOARD_WIDTH as f32
                    || x < start_x
                    || y < start_y
                {
                    println!("Out of bounds...");
                    return;
                }

                // Calculates list index (if in bounds) of the clicked tile
                let x_tile = ((x - start_x) / TILE_SIZE as f32) as usize;
                let y_tile = ((y - start_y) / TILE_SIZE as f32) as usize;

				let mut index = y_tile * BOARD_SIZE + x_tile;
				if self.flipped_board {
					index = flip_index(&(index as i32), BOARD_SIZE as i32) as usize;
				}

				// Attempts to grab a piece from the given tile
                if let Some(piece) = self.board[index].take() {
                    self.grabbed_piece = Some((piece, (x_tile, y_tile)));
                } else {
                    return
                }
            }
            _ => {}
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {

    }
}
