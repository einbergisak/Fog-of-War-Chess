
use ggez::{Context, GameResult, event::{EventHandler, KeyCode, KeyMods, MouseButton}, graphics::{self, DrawParam, Image, spritebatch::SpriteBatch}, nalgebra::Point2};

use crate::{Game, SCREEN_HEIGHT, SCREEN_WIDTH, STATE, enter_name_screen::on_key_down, game::{BACKGROUND_COLOR, LIGHT_COLOR}, menu::{clickable::ClickableGroup, menu_state::Menu}, piece::{get_piece_rect, get_valid_move_indices, Piece}, render_utilities::{flip_board, flip_index, translate_to_index}};
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
            let state_read = STATE.get().read().unwrap().clone();

            let incoming_move = state_read.incoming_move;
            match incoming_move {
                Some(move_) => {
                    self.move_piece_from_board(move_);
                    // After move has been performed we remove the values
                    STATE.get().write().unwrap().incoming_move = None;
                }
                None => {}
            }

            if self.active_turn {
                if let Some((piece, piece_dest_index)) = self.premove.take() {
                    if let Some(piece) = self.board[piece.get_index()].take() {
                        self.attempt_move(piece, piece_dest_index)
                    } else {
                        self.board[piece.get_index()] = Some(piece)
                    }
                }
            }
        }

            // Check if network state has updated
            let event_validation = state_read.event_validation;

            if self.menu.visible || self.winner.is_some() {

                // Check if lobbies have changed
                if self.lobby_sync != state_read.lobby_sync {
                    self.menu.clear_list_items_from_list();
                    self.menu.generate_list_item_from_list(&state_read.lobbies);
                    self.lobby_sync = state_read.lobby_sync;
                }

                if event_validation.create_room {
                    self.menu.visible = false;
                    self.active_turn = true;
                    self.playing_as_white = true;
                    self.is_admin = true;
                    self.update_available_moves();
                    STATE.get().write().unwrap().event_validation.create_room = false;
                }

                if event_validation.join_room {
                    self.menu.visible = false;

                    // Send name to opponent
                    self.connection.send("send_name", "");
                    // Ask server for opponent name
                    self.connection.send("get_opponent_name", "");
                    self.update_available_moves();
                    STATE.get().write().unwrap().event_validation.join_room = false;
                }

                if event_validation.play_again {
                    self.reset_game();
                    self.playing_as_white = !self.playing_as_white;
                    self.active_turn = self.playing_as_white;
                    STATE.get().write().unwrap().event_validation.play_again = false;
                }
            }

            if event_validation.opponent_connect {
                println!("Opponent connect parsed!");
                // If the user is still in end game screen we force him into the game
                if self.winner.is_some() {
                    self.reset_game();
                    self.playing_as_white = !self.playing_as_white;
                    self.active_turn = self.playing_as_white;
                }

                let color = if self.playing_as_white {
                    String::from("black")
                } else {
                    String::from("white")
                };
                self.connection.send("set_opponent_color", &color);
                STATE.get().write().unwrap().event_validation.opponent_connect = false;
            }

            if event_validation.opponent_disconnect {
                STATE.get().write().unwrap().event_validation.opponent_disconnect = false;
            }

            match event_validation.set_color {
                Some(White) => {
                    self.playing_as_white = true;
                    self.active_turn = true;
                    STATE.get().write().unwrap().event_validation.set_color = None;
                }
                Some(Black) => {
                    self.playing_as_white = false;
                    self.active_turn = false;
                    STATE.get().write().unwrap().event_validation.set_color = None;
                }
                _ => {}
            }
            if event_validation.resign {
                let winner = if self.playing_as_white { White } else { Black };
                self.game_over(winner);
                STATE.get().write().unwrap().event_validation.resign = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let read_state = STATE.get().read().unwrap().clone();

        // Draw background
        match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT),
            graphics::Color::from(BACKGROUND_COLOR),
        ) {
            Ok(background) => {
                graphics::draw(ctx, &background, graphics::DrawParam::default())
                    .expect("Could not render background");
            }
            Err(_) => {}
        }

        if read_state.entering_name {
            self.menu.render_name_interface(ctx);
            self.menu.draw_clickables(ctx, vec![ClickableGroup::EnterName]);
            return graphics::present(ctx);
        }

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

        // Draw room code
        if let Some(id) = &read_state.room_id {
            self.menu.draw_text(
                ctx, format!("Room code: {}", id.clone().replace("\"", "")),
                (
                    BOARD_ORIGO_X,
                    50.0 / 2.0 - 40.0 / 2.0
                ),
                (
                    BOARD_WIDTH as f32,
                    40.0
                ),
                graphics::Color::from(LIGHT_COLOR),
                graphics::Align::Right
            );
      }

        render_utilities::render_movement_indication(&self, ctx)?;

        render_utilities::render_fog_and_pieces(&self, ctx)?;

        piece::promotion::render_promotion_interface(&self, ctx)?;

        // Draw opponent name
        let opponent_name = read_state.event_validation.opponent_name.clone();
        if let Some(name) = opponent_name {
            self.menu.draw_text(
                ctx,
                name,
                (
                    BOARD_ORIGO_X,
                    50.0 / 2.0 - 40.0 / 2.0
                ),
                (
                    BOARD_WIDTH as f32,
                    40.0  // Same height as the room code text
                ),
                graphics::Color::from(LIGHT_COLOR),
                graphics::Align::Left
            );
        } else {
            self.menu.draw_text(
                ctx,
                String::from("Awaiting player..."),
                (
                    BOARD_ORIGO_X,
                    50.0 / 2.0 - 40.0 / 2.0
                ),
                (
                    BOARD_WIDTH as f32,
                    40.0  // Same height as the room code text
                ),
                graphics::Color::from(LIGHT_COLOR),
                graphics::Align::Left
            );
        }

        // Draw name
        let name = read_state.name.clone();
        self.menu.draw_text(
            ctx,
            name,
            (
                BOARD_ORIGO_X,
                SCREEN_HEIGHT - 50.0 / 2.0 - 40.0 / 2.0
            ),
            (
                BOARD_WIDTH as f32,
                40.0  // Same height as the room code text
            ),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Left
        );

        self.menu.draw_clickables(ctx, vec![ClickableGroup::InGame]);

        // Draw game over menu
        if self.winner.is_some() {
            self.menu.render_game_over(ctx, self.winner);
        }

        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {

                let read_state = STATE.get().read().unwrap().clone();

                // UI logic

                let mut parsing_groups: Vec<ClickableGroup> = Vec::new();
                if read_state.entering_name {
                    parsing_groups.push(ClickableGroup::EnterName);
                }
                if self.menu.visible {
                    parsing_groups.push(ClickableGroup::MainMenu);
                    parsing_groups.push(ClickableGroup::MainMenuList);
                } else if self.winner.is_some() {
                    parsing_groups.push(ClickableGroup::GameOverMenu);
                } else {
                    parsing_groups.push(ClickableGroup::InGame);
                }
                // Button logic
                self.button_parsing(parsing_groups);

                if self.menu.visible || self.winner.is_some() {
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
                if let Some(piece) = self.selected_piece.take() {
                    let mut piece_dest_index = translate_to_index(x_tile, y_tile);

                    if self.playing_as_white {
                        piece_dest_index = flip_index(piece_dest_index);
                    }

                    if let Some(piece) = self.board[piece.get_index()].take() {
                        self.attempt_move(piece, piece_dest_index);
                    }

                    // Prevents attempting to grab a piece which has just been unselected
                    if piece.index == index {
                        return;
                    }
                }
                // Attempt to grab a piece from the clicked tile
                if let Some(piece) = self.board[index].clone().take() {
                    match &piece.color {
                        PieceColor::White if !self.playing_as_white => {
                            return;
                        }
                        PieceColor::Black if self.playing_as_white => {
                            return;
                        }
                        _ => {}
                    }
                    self.grabbed_piece = Some(piece);
                    // Lock the cursor inside the application
                    ggez::input::mouse::set_cursor_grabbed(ctx, true).expect("Cursor grab failed");
                    ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Hand)
                } else {
                    // Lets you cancel your premove by clicking on something that's not interactive
                    if let Some(_) = self.premove {
                        self.premove = None;
                    }
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
                if self.menu.visible || self.winner.is_some() {
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
                        // If we are out of bounds then the grab is cancelled
                        println!("Out of bounds");
                        return;
                    }
                    if let Some(piece) = self.board[piece.get_index()].take() {
                        self.attempt_move(piece, piece_dest_index);
                    }
                } else {
                    return;
                }
            }
            _ => {}
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {

        let mut active_groups: Vec<ClickableGroup> = Vec::new();
        // Entering name state
        if STATE.get().read().unwrap().entering_name {
            active_groups.push(ClickableGroup::EnterName);
        } // End game screen
        else if self.winner.is_some() {
            active_groups.push(ClickableGroup::GameOverMenu);
        } // Main menu screen
        else if self.menu.visible {
            active_groups.push(ClickableGroup::MainMenu);
            active_groups.push(ClickableGroup::MainMenuList);
        }
        else { // In game
            active_groups.push(ClickableGroup::InGame);
        }

        self.menu.on_mouse_move(ctx, x, y, active_groups);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) {
        self.menu.on_mouse_wheel(ctx, y);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        on_key_down(keycode, keymods, repeat);
    }
}
