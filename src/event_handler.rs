use ggez::{Context, GameResult, event::{EventHandler, MouseButton}, graphics::{self, DrawParam, Image, spritebatch::SpriteBatch}, nalgebra::{Point2}};

use crate::{Game, SCREEN_HEIGHT, SCREEN_WIDTH, STATE, piece::{get_piece_rect, get_valid_move_indices, Piece}, render_utilities::{flip_board, flip_index, translate_to_index}};

pub(crate) const BOARD_SIZE: usize = 8;
pub(crate) const TILE_SIZE: i32 = 100;
pub(crate) const BOARD_WIDTH: i32 = BOARD_SIZE as i32 * TILE_SIZE;

pub(crate) const BOARD_ORIGO_X: f32 = SCREEN_WIDTH / 2.0 - (BOARD_WIDTH / 2) as f32;
pub(crate) const BOARD_ORIGO_Y: f32 = SCREEN_HEIGHT / 2.0 - (BOARD_WIDTH / 2) as f32;

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {
            let incoming_move = STATE.get().read().unwrap().incoming_move;
            match incoming_move {
                Some((target, to)) => {
                    self.move_piece_index(target, to);
                    // After move has been performed we remove the values
                    STATE.get().write().unwrap().incoming_move = None;
                }
                None => {}
            }
        }

        if self.lobby_sync != STATE.get().read().unwrap().lobby_sync {
            self.menu.clear_list_items_from_list();
            self.menu.generate_list_item_from_list(&STATE.get().read().unwrap().lobbies);
            self.lobby_sync = STATE.get().read().unwrap().lobby_sync;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // If menu is active we don't bother showing the rest of the game
        if self.menu.visible {
            self.menu.render(ctx);
            return graphics::present(ctx)
        }

        // Draws the background board
        graphics::draw(ctx, &self.board_mesh, (Point2::<f32>::new(BOARD_ORIGO_X, BOARD_ORIGO_Y),))?;

        let piece_image = Image::new(ctx, "/pieces.png")?;
        let mut piece_batch = SpriteBatch::new(piece_image);

        let render_board = if self.playing_as_white {
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
            let (x, y) = (
                ggez::input::mouse::position(ctx).x,
                ggez::input::mouse::position(ctx).y,
            );
            grabbed_param = Some(DrawParam::default().src(rect).dest(Point2::new(
                x - TILE_SIZE as f32 / 2.0,
                y - TILE_SIZE as f32 / 2.0,
            )));
        } else {
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
                        (x as f32) * TILE_SIZE as f32 + BOARD_ORIGO_X,
                        (y as f32) * TILE_SIZE as f32 + BOARD_ORIGO_Y,
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

                 // UI logic
                if self.menu.visible {
                    self.button_parsing();
                    return
                }

                //------------------------------------------------------

                let (start_x, start_y) = (BOARD_ORIGO_X, BOARD_ORIGO_Y);

                // Cursor out of bounds checking
                if x > start_x + BOARD_WIDTH as f32
                    || y > start_y + BOARD_WIDTH as f32
                    || x < start_x
                    || y < start_y
                {
                    println!("CLICK OUTSIDE {}", x);
                    return;
                }

                // Calculates list index (if cursor is in bounds) of the clicked tile
                let x_tile = ((x - start_x) / TILE_SIZE as f32) as usize;
                let y_tile = ((y - start_y) / TILE_SIZE as f32) as usize;

                let mut index = translate_to_index(x_tile, y_tile);
                if self.playing_as_white {
                    index = flip_index(&(index as i32), BOARD_SIZE as i32) as usize;
                }

                // Attempts to grab a piece from the given tile
                if let Some(piece) = self.board[index].take() {
                    match &piece.color {
                        crate::piece::Color::White if !self.playing_as_white => {
                            self.board[index] = Some(piece);
                            return
                        }
                        crate::piece::Color::Black if self.playing_as_white => {
                            self.board[index] = Some(piece);
                            return
                        }
                        _ => {}
                    }
                    self.grabbed_piece = Some((piece, (x_tile, y_tile)));

                    println!("Grabbed piece at ({}, {})", x_tile, y_tile);
                    // Lock the cursor inside the application
                    ggez::input::mouse::set_cursor_grabbed(ctx, true).expect("Cursor grab failed");
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

                // UI logic
                if self.menu.visible {
                    return
                }

                //------------------------------------------------------

                ggez::input::mouse::set_cursor_grabbed(ctx, false).expect("Cursor release fail");
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);

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
                let (start_x, start_y) = (BOARD_ORIGO_X, BOARD_ORIGO_Y);

                // Calculates list index (if in bounds) of the clicked tile
                let x_tile = ((x - start_x) / TILE_SIZE as f32) as usize;
                let y_tile = ((y - start_y) / TILE_SIZE as f32) as usize;

                let mut piece_source_index = translate_to_index(source_x as usize, source_y as usize);
                let mut piece_dest_index = translate_to_index(x_tile, y_tile);

                if self.playing_as_white {
                    piece_dest_index =
                        flip_index(&(piece_dest_index as i32), BOARD_SIZE as i32) as usize;
                    piece_source_index = flip_index(&(piece_source_index as i32), BOARD_SIZE as i32) as usize;
                }

                // Out of bounds checking
                if x - start_x > BOARD_WIDTH as f32
                    || y - start_y > BOARD_WIDTH as f32
                    || x < start_x
                    || y < start_y
                {
                    // If we are out of bounds then we place the piece
                    // at its original position

                    // Board index for the piece which the cursor is on
                    self.board[piece_source_index] = Some(piece);
                    println!("Out of bounds");
                    return;
                }

                let valid_moves = get_valid_move_indices(self, &piece, piece_source_index);
                println!("Valid moves: {:?}", valid_moves);
                if valid_moves.contains(&piece_dest_index) && self.active_turn {
                    println!("Move to index {} is valid", piece_dest_index);
                    self.move_piece(piece, piece_dest_index);
                    self.connection.send(
                        "opponent",
                        &format!("{}:{}", piece_source_index, piece_dest_index),
                    );

                } else {
                    println!("Move to index {} is NOT valid", piece_dest_index);
                    // // Reset position to source
                    self.board[piece_source_index] = Some(piece);
                }
            }
            _ => {}
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.menu.on_mouse_move(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) {
        self.menu.on_mouse_wheel(ctx, y);
    }
}
