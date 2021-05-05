use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{
        self, spritebatch::SpriteBatch, Color, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect,
    },
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{Game, piece::{Piece, get_piece_rect, get_valid_move_indices, is_valid_move}, render_utilities::{flip_board, flip_index, translate_to_index}};

pub(crate) const BOARD_SIZE: usize = 8;
pub(crate) const TILE_SIZE: i32 = 100;
pub(crate) const BOARD_WIDTH: i32 = BOARD_SIZE as i32 * TILE_SIZE;

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {}
        Ok(())
    }
    // y * 8 + x
    // (y-1) * 8 + x - 1
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Draws the background board
        graphics::draw(ctx, &self.board_mesh, (Point2::<f32>::new(0.0, 0.0),))?;

        let piece_image = Image::new(ctx, "/pieces.png")?;
        let mut piece_batch = SpriteBatch::new(piece_image);

        let render_board = if self.flipped_board {
            flip_board(&self.board)
        } else {
            self.board.clone()
        };

        let grabbed_index: Option<usize>;
        let grabbed_param: Option<DrawParam>;
        // Renders the grabbed piece
        if let Some((piece, (x, y))) = &self.grabbed_piece {
            grabbed_index = Some(translate_to_index(x.clone(), y.clone()));

            let rect = get_piece_rect(&piece);
            let (mut x, mut y) = (
                ggez::input::mouse::position(ctx).x,
                ggez::input::mouse::position(ctx).y,
            );
            grabbed_param = Some(DrawParam::default().src(rect).dest(Point2::new(
                x - TILE_SIZE as f32 / 2.0,
                y - TILE_SIZE as f32 / 2.0,
            )));
        } else{
            grabbed_index = None;
            grabbed_param = None;
        };

        // Render each piece in the board
        for (index, tile) in render_board.iter().enumerate() {
            if let Some(g_i) = &grabbed_index {
                if index == *g_i {
                    continue;
                }
            }
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

        if let Some(param) = grabbed_param {
            piece_batch.add(param);
        }

        graphics::draw(ctx, &piece_batch, (Point2::<f32>::new(0.0, 0.0),))?;

        // Draw pieces
        //let mut piece_batch = SpriteBatch::new()
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                let (start_x, start_y) = (0.0, 0.0);

                // Out of bounds checking
                if start_x + x > BOARD_WIDTH as f32
                    || start_y + y > BOARD_WIDTH as f32
                    || x < start_x
                    || y < start_y
                {
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
                if let Some(piece) = self.board[index].clone() {
                    self.grabbed_piece = Some((piece, (x_tile, y_tile)));
                    // Lock the cursor inside the application
                    ggez::input::mouse::set_cursor_grabbed(ctx, true)
                        .expect("Cursor grabbed failed");
                    ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Hand)
                } else {
                    return;
                }
            }
            _ => {}
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        match button {
            MouseButton::Left => {
                let piece: Piece;
                let source_x;
                let source_y;
                if let Some((internal_piece, (internal_x, internal_y))) = self.grabbed_piece.take()
                {
                    piece = internal_piece;
                    source_x = internal_x as f32;
                    source_y = internal_y as f32;
                } else {
                    return;
                }
                let (start_x, start_y) = (0.0, 0.0);

                // Calculates list index (if in bounds) of the clicked tile
                let x_tile = ((x - start_x) / TILE_SIZE as f32) as usize;
                let y_tile = ((y - start_y) / TILE_SIZE as f32) as usize;

                // Out of bounds checking
                if start_x + x > BOARD_WIDTH as f32
                    || start_y + y > BOARD_WIDTH as f32
                    || x < start_x
                    || y < start_y
                {
                    // If we are out of bounds then we place the piece
                    // at its original position

                    let mut index = source_y as usize * BOARD_SIZE + source_x as usize;
                    if self.flipped_board {
                        index = flip_index(&(index as i32), BOARD_SIZE as i32) as usize;
                    }

                    self.board[index] = Some(piece);
                    return;
                }

                let mut index = y_tile * BOARD_SIZE + x_tile;
                if self.flipped_board {
                    index = flip_index(&(index as i32), BOARD_SIZE as i32) as usize;
                }
                ggez::input::mouse::set_cursor_grabbed(ctx, false);
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);
                let piece_index;

                piece_index = translate_to_index(source_x as usize, source_y as usize);

                let temp = get_valid_move_indices(&self.board, piece_index);
                println!("HERE IT IS :D{:?}", temp);
                if is_valid_move(index, temp) {
                    self.board[piece_index] = Some(piece);
                } else {
                    // TODO this is copied code from row 150, make this better
                    // Reset position to start
                    index = source_y as usize * BOARD_SIZE + source_x as usize;
                    if self.flipped_board {
                        index = flip_index(&(index as i32), BOARD_SIZE as i32) as usize;
                    }

                    self.board[index] = Some(piece);
                }
            }
            _ => {}
        }
    }
}