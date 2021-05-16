use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, Font, Text},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{
    game::{BACKGROUND_COLOR, LIGHT_COLOR},
    piece::{self, piece::PieceColor},
    render_utilities::{self, flip_index, translate_to_index},
    Game, SCREEN_HEIGHT, SCREEN_WIDTH, STATE,
};

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
                Some(move_) => {
                    self.move_piece_from_board(move_);
                    // After move has been performed we remove the values
                    STATE.get().write().unwrap().incoming_move = None;
                }
                None => {}
            }
        }

        // Check if lobbies have changed
        if self.menu.visible {
            if self.lobby_sync != STATE.get().read().unwrap().lobby_sync {
                self.menu.clear_list_items_from_list();
                self.menu
                    .generate_list_item_from_list(&STATE.get().read().unwrap().lobbies);
                self.lobby_sync = STATE.get().read().unwrap().lobby_sync;
            }

            // Check if network state has updated
            let event_validation = &STATE.get().read().unwrap().event_validation;
            if event_validation.create_room {
                self.menu.visible = false;
                self.active_turn = true;
                self.playing_as_white = true;
                println!("CREATE ROOM RESPONSE OK!");

                self.update_available_moves();

                //STATE.get().write().unwrap().event_validation.create_room = false;
            } else if event_validation.join_room {
                self.menu.visible = false;

                self.update_available_moves();

                //STATE.get().write().unwrap().event_validation.join_room = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Draw background
        let background = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT),
            graphics::Color::from(BACKGROUND_COLOR),
        )
        .expect("Could not render list");

        graphics::draw(ctx, &background, graphics::DrawParam::default())
            .expect("Could not render background");

        // If menu is active we don't bother showing the rest of the game
        if self.menu.visible {
            self.menu.render(ctx);
            return graphics::present(ctx);
        }

        // Draws the background board
        graphics::draw(
            ctx,
            &self.board_mesh,
            (Point2::<f32>::new(BOARD_ORIGO_X, BOARD_ORIGO_Y),),
        )?;

        if let Some(id) = &STATE.get().read().unwrap().room_id {
            let mut text = Text::new(format!("Room code: {}", id.clone().replace("\"", "")));
            let font = Font::new(ctx, "/fonts/Roboto-Regular.ttf").expect("Error loading font");
            let scale = 40.0;
            text.set_font(font, graphics::Scale::uniform(scale));

            text.set_bounds(Point2::new(SCREEN_WIDTH, 40.0), graphics::Align::Center);

            graphics::draw(
                ctx,
                &text,
                graphics::DrawParam::default()
                    .dest(Point2::<f32>::new(0.0, 0.0))
                    .color(graphics::Color::from(LIGHT_COLOR)),
            )
            .expect("Error drawing clickable text");
        }

        render_utilities::render_movement_indication(&self, ctx)?;

        render_utilities::render_fog_and_pieces(&self, ctx)?;

        piece::promotion::render_promotion_interface(&self, ctx)?;

        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                // UI logic
                if self.menu.visible {
                    self.button_parsing();
                    return;
                }

                //------------------------------------------------------

                // Cursor out of bounds checking
                if x > BOARD_ORIGO_X + BOARD_WIDTH as f32
                    || y > BOARD_ORIGO_Y + BOARD_WIDTH as f32
                    || x < BOARD_ORIGO_X
                    || y < BOARD_ORIGO_Y
                {
                    println!("CLICK OUTSIDE {}", x);
                    return;
                }
                // Calculates (on screen) list index (if cursor is in bounds) of the clicked tile
                let x_tile = (x - BOARD_ORIGO_X) as usize / TILE_SIZE as usize;
                let y_tile = (y - BOARD_ORIGO_Y) as usize / TILE_SIZE as usize;

                let mut index = translate_to_index(x_tile, y_tile);
                if self.playing_as_white {
                    index = flip_index(index);
                }

                // Pawn promotion interface
                piece::promotion::check_promotion(self, x_tile, y_tile);

                // If a piece has been selected by clicking, try to move to the clicked tile
                if let Some(piece) = self.selected_piece {
                    let mut piece_dest_index = translate_to_index(x_tile, y_tile);

                    if self.playing_as_white {
                        piece_dest_index = flip_index(piece_dest_index);
                    }

                    self.attempt_move(piece, piece_dest_index);
                    self.selected_piece = None;
                }
                // Otherwise, attempt to grab a piece from the clicked tile
                else if let Some(piece) = self.board[index].take() {
                    match &piece.color {
                        PieceColor::White if !self.playing_as_white => {
                            self.board[index] = Some(piece);
                            return;
                        }
                        PieceColor::Black if self.playing_as_white => {
                            self.board[index] = Some(piece);
                            return;
                        }
                        _ => {}
                    }
                    self.grabbed_piece = Some(piece);
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
                    return;
                }

                //------------------------------------------------------

                ggez::input::mouse::set_cursor_grabbed(ctx, false).expect("Cursor release fail");
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);
                if let Some(piece) = self.grabbed_piece.take() {
                    let (start_x, start_y) = (BOARD_ORIGO_X, BOARD_ORIGO_Y);

                    // Calculates list index (if in bounds) of the clicked tile
                    let x_tile = ((x - start_x) / TILE_SIZE as f32) as usize;
                    let y_tile = ((y - start_y) / TILE_SIZE as f32) as usize;

                    let piece_source_index = piece.index;
                    let mut piece_dest_index = translate_to_index(x_tile, y_tile);

                    if self.playing_as_white {
                        piece_dest_index = flip_index(piece_dest_index);
                    }

                    // If the cursor is released on the same tile as it was grabbed on, go into "click & select" mode instead of "drag & drop" mode
                    if piece_dest_index == piece_source_index {
                        self.selected_piece = Some(piece);
                        return;
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

                    self.attempt_move(piece, piece_dest_index);
                } else {
                    return;
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
