use std::time::{Duration, Instant};

use ggez::{
    event::{EventHandler, MouseButton},
    graphics,
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{
    game::{BACKGROUND_COLOR, LIGHT_COLOR},
    menu::clickable::ClickableGroup,
    render_utilities::{flip_index, translate_to_index},
    Game, SCREEN_HEIGHT, SCREEN_WIDTH, STATE,
};

use ggez::timer;

use crate::piece::piece::PieceColor;

use crate::{
    piece::{self, piece::PieceColor::*},
    render_utilities,
};

pub(crate) const BOARD_SIZE: usize = 8;
pub(crate) const TILE_SIZE: i32 = 100;
pub(crate) const BOARD_WIDTH: i32 = BOARD_SIZE as i32 * TILE_SIZE;

pub(crate) const BOARD_ORIGO_X: f32 = SCREEN_WIDTH / 2.0 - (BOARD_WIDTH / 2) as f32;
pub(crate) const BOARD_ORIGO_Y: f32 = SCREEN_HEIGHT / 2.0 - (BOARD_WIDTH / 2) as f32;

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {
            if self.game_active {
                if self.active_turn {
                    if self.time.current_time_left < self.time.turn_start.elapsed() {
                        if self.playing_as_white {
                            self.game_over(White);
                            return Ok(());
                        } else {
                            self.game_over(Black);
                            return Ok(());
                        }
                    }
                } else {
                    if self.time.opponent_time_left < self.time.turn_start.elapsed() {
                        if self.playing_as_white {
                            self.game_over(Black);
                            return Ok(());
                        } else {
                            self.game_over(White);
                            return Ok(());
                        }
                    }
                }
            }

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
                    self.time.turn_start = Instant::now();
                    self.update_available_moves();
                    STATE.get().write().unwrap().event_validation.create_room = false;
                }

                // This user successfully joined the room
                if event_validation.join_room {
                    self.menu.visible = false;

                    // Send name to opponent
                    self.connection.send("send_name", "");
                    // Ask server for opponent name
                    self.connection.send("get_opponent_name", "");
                    self.update_available_moves();
                    STATE.get().write().unwrap().opponent_online = true;
                    STATE.get().write().unwrap().event_validation.join_room = false;
                }

                if event_validation.play_again {
                    self.reset_game();
                    self.playing_as_white = !self.playing_as_white;
                    self.active_turn = self.playing_as_white;
                    self.time.turn_start = Instant::now();

                    self.update_available_moves();
                    STATE.get().write().unwrap().event_validation.play_again = false;
                }
            }

            // A new connection joined the game
            if event_validation.opponent_connect {
                println!("Opponent connect parsed!");
                // If the user is still in end game screen we force him into the game
                if self.winner.is_some() {
                    self.reset_game();
                    self.playing_as_white = !self.playing_as_white;
                    self.active_turn = self.playing_as_white;
                    self.time.turn_start = Instant::now();
                }

                let color = if (self.winner.is_some() && self.playing_as_white)
                    || (self.winner.is_none() && !self.playing_as_white)
                {
                    String::from("white")
                } else {
                    String::from("black")
                };

                // Update available moves when client connects
                self.update_available_moves();

                // Tell the new connection which color it should have
                // And what the clock should start at
                self.connection.send("set_opponent_color", &color);
                self.connection.send(
                    "set_clock_time",
                    &format!(
                        "{}:{}",
                        self.time.initial_time.as_secs(),
                        self.time.increment.as_secs()
                    )[..],
                );

                STATE
                    .get()
                    .write()
                    .unwrap()
                    .event_validation
                    .opponent_connect = false;
            }

            if event_validation.opponent_disconnect {
                if self.playing_as_white {
                    self.game_over(PieceColor::White);
                } else {
                    self.game_over(PieceColor::Black);
                }

                // Clear opponent
                STATE.get().write().unwrap().event_validation.opponent_name = None;

                STATE
                    .get()
                    .write()
                    .unwrap()
                    .event_validation
                    .opponent_disconnect = false;
            }

            match event_validation.set_color {
                Some(White) => {
                    self.playing_as_white = true;
                    self.active_turn = true;
                    self.time.turn_start = Instant::now();

                    self.update_available_moves();
                    STATE.get().write().unwrap().event_validation.set_color = None;
                }
                Some(Black) => {
                    self.playing_as_white = false;
                    self.active_turn = false;
                    self.time.turn_start = Instant::now();

                    self.update_available_moves();
                    STATE.get().write().unwrap().event_validation.set_color = None;
                }
                _ => {}
            }
            if event_validation.resign {
                let winner = if self.playing_as_white { White } else { Black };
                self.game_over(winner);
                STATE.get().write().unwrap().event_validation.resign = false;
            }

            match event_validation.time {
                Some((total_time, increment)) => {
                    self.time.time_set = true;
                    self.time.initial_time = Duration::from_secs(total_time);
                    self.time.increment = Duration::from_secs(increment);

                    self.time.current_time_left = self.time.initial_time;
                    self.time.opponent_time_left = self.time.initial_time;
                    STATE.get().write().unwrap().event_validation.time = None;
                }
                None => {}
            }

            if event_validation.deselect_cursor {
                ggez::input::mouse::set_cursor_grabbed(ctx, false)
                    .expect("Could not deselect cursor");
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);
                STATE
                    .get()
                    .write()
                    .unwrap()
                    .event_validation
                    .deselect_cursor = false;
            }
        }

        // Let the loop sleep until read to continue
        // Prevents the program from taking up
        // 100% CPU power if not neccessary
        timer::yield_now();
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
            self.menu
                .draw_clickables(ctx, vec![ClickableGroup::EnterName]);
            return graphics::present(ctx);
        }

        // If menu is active we don't bother showing the rest of the game
        if self.menu.visible {
            self.menu.render(ctx);
            return graphics::present(ctx);
        }

        if !self.time.time_set {
            self.render_time_interface(ctx);
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
                ctx,
                format!("Room code: {}", id.clone().replace("\"", "")),
                (BOARD_ORIGO_X, 50.0 / 2.0 - 40.0 / 2.0),
                (BOARD_WIDTH as f32, 40.0),
                graphics::Color::from(LIGHT_COLOR),
                graphics::Align::Right,
            );
        }

        render_utilities::render_movement_indication(&self, ctx)?;

        render_utilities::render_fog_and_pieces(&self, ctx)?;

        piece::promotion::render_promotion_interface(&self, ctx)?;

        // Draw opponent name
        let mut display_name = String::from("Awaiting player...");
        let opponent_name = read_state.event_validation.opponent_name.clone();
        if let Some(name) = opponent_name {
            display_name = name;
        }

        self.menu.draw_text(
            ctx,
            display_name,
            (BOARD_ORIGO_X, 50.0 / 2.0 - 40.0 / 2.0),
            (
                BOARD_WIDTH as f32,
                40.0, // Same height as the room code text
            ),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Left,
        );

        // Draw name
        let name = read_state.name.clone();
        self.menu.draw_text(
            ctx,
            name,
            (BOARD_ORIGO_X, SCREEN_HEIGHT - 50.0 / 2.0 - 40.0 / 2.0),
            (
                BOARD_WIDTH as f32,
                40.0, // Same height as the room code text
            ),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Left,
        );

        self.render_time(ctx);

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
                } else if self.menu.visible {
                    parsing_groups.push(ClickableGroup::MainMenu);
                    parsing_groups.push(ClickableGroup::MainMenuList);
                } else if self.winner.is_some() {
                    parsing_groups.push(ClickableGroup::GameOverMenu);
                } else if !self.time.time_set {
                    parsing_groups.push(ClickableGroup::TimeSelection);
                } else {
                    parsing_groups.push(ClickableGroup::InGame);
                }
                // Button logic
                self.button_parsing(parsing_groups);

                if read_state.entering_name
                    || self.menu.visible
                    || self.winner.is_some()
                    || !self.time.time_set
                {
                    return;
                }

                //------------------------------------------------------

                // If there is no opponent we cannot make moves
                if !read_state.opponent_online {
                    return;
                }

                // Cursor out of bounds checking
                if x > BOARD_ORIGO_X + BOARD_WIDTH as f32
                    || y > BOARD_ORIGO_Y + BOARD_WIDTH as f32
                    || x < BOARD_ORIGO_X
                    || y < BOARD_ORIGO_Y
                {
                    // Lets you cancel your premoves by clicking on something that's not interactive
                    self.premove = None;
                    self.selected_piece = None;
                    return;
                }
                // Calculates (on screen) list index (if cursor is in bounds) of the clicked tile
                let x_tile = (x - BOARD_ORIGO_X) as usize / TILE_SIZE as usize;
                let y_tile = (y - BOARD_ORIGO_Y) as usize / TILE_SIZE as usize;

                let mut clicked_index = translate_to_index(x_tile, y_tile);
                if self.playing_as_white {
                    clicked_index = flip_index(clicked_index);
                }

                // Pawn promotion interface
                piece::promotion::check_promotion(self, x_tile, y_tile);

                let mut had_selected = false;

                // If a piece has been selected by clicking, try to move to the clicked tile
                if let Some(piece) = self.selected_piece.take() {
                    had_selected = true;
                    let mut piece_dest_index = translate_to_index(x_tile, y_tile);

                    if self.playing_as_white {
                        piece_dest_index = flip_index(piece_dest_index);
                    }

                    // If the player has selected a piece that's present on the board, attempt to move
                    if let Some(piece) = self.board[piece.get_index()].take() {
                        self.attempt_move(piece, piece_dest_index);
                    }

                    // Prevents attempting to grab a piece which has just been unselected
                    if piece.index == clicked_index {
                        return;
                    }
                }

                // Attempt to grab a piece from the clicked tile
                if let Some(piece) = self.board[clicked_index].clone().take() {
                    // Prevents you from grabbing the piece you just premoved
                    if let Some((p, _d)) = &self.premove {
                        if p.get_index() == clicked_index {
                            return;
                        }
                    }

                    if let Some(m) = self.move_history.last() {
                        // Prevents the player from grabbing directly after making a move by selecting-by-clicking
                        if m.piece_dest_index == piece.get_index() && had_selected {
                            return;
                        }
                    }
                    match &piece.color {
                        White if !self.playing_as_white => {
                            // Cancel premoves if attempting to select an opposing piece
                            if !had_selected {
                                self.premove.take();
                            }

                            return;
                        }
                        Black if self.playing_as_white => {
                            // Cancel premoves if attempting to select an opposing piece
                            if !had_selected {
                                self.premove.take();
                            }
                            return;
                        }
                        _ => {}
                    }
                    self.grabbed_piece = Some(piece);
                    // Lock the cursor inside the application
                    ggez::input::mouse::set_cursor_grabbed(ctx, true).expect("Cursor grab failed");
                    ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Hand)
                }
                // If a piece was selected when this function was called, don't interpret a the click as a premove cancel
                else if !had_selected {
                    // Lets you cancel your premoves by clicking on something that's not interactive
                    self.premove.take();
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
                if self.menu.visible || self.winner.is_some() || !self.time.time_set {
                    return;
                }

                //------------------------------------------------------

                ggez::input::mouse::set_cursor_grabbed(ctx, false).expect("Cursor release fail");
                ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);

                if let Some(piece) = self.grabbed_piece.take() {
                    // Calculates list index (if in bounds) of the clicked tile
                    let x_tile = ((x - BOARD_ORIGO_X) / TILE_SIZE as f32) as usize;
                    let y_tile = ((y - BOARD_ORIGO_Y) / TILE_SIZE as f32) as usize;

                    let mut piece_dest_index = translate_to_index(x_tile, y_tile);
                    let piece_source_index = piece.index;

                    if self.playing_as_white {
                        piece_dest_index = flip_index(piece_dest_index);
                    }

                    // If the cursor is released on the same tile as it was grabbed on, go into "click & select" mode instead of "drag & drop" mode
                    if piece_dest_index == piece_source_index {
                        self.selected_piece = Some(piece);
                        return;
                    }

                    // Out of bounds checking
                    if x - BOARD_ORIGO_X > BOARD_WIDTH as f32
                        || y - BOARD_ORIGO_Y > BOARD_WIDTH as f32
                        || x < BOARD_ORIGO_X
                        || y < BOARD_ORIGO_Y
                    {
                        // If we are out of bounds then the grab is cancelled
                        println!("Out of bounds");
                        return;
                    }
                    if self.premove.is_some() {
                        self.attempt_move(piece, piece_dest_index);
                    } else if let Some(piece) = self.board[piece.get_index()].take() {
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
        let read_state = STATE.get().read().unwrap();
        // Entering name state
        let mut parsing_groups: Vec<ClickableGroup> = Vec::new();
        if read_state.entering_name {
            parsing_groups.push(ClickableGroup::EnterName);
        } else if self.menu.visible {
            parsing_groups.push(ClickableGroup::MainMenu);
            parsing_groups.push(ClickableGroup::MainMenuList);
        } else if self.winner.is_some() {
            parsing_groups.push(ClickableGroup::GameOverMenu);
        } else if !self.time.time_set {
            parsing_groups.push(ClickableGroup::TimeSelection);
        } else {
            parsing_groups.push(ClickableGroup::InGame);
        }

        self.menu.on_mouse_move(ctx, x, y, parsing_groups);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) {
        self.menu.on_mouse_wheel(ctx, y);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) {
        // Name input when the game is launched
        if STATE.get().read().unwrap().entering_name {
            let mut name = STATE.get().read().unwrap().name.clone();

            // 8u8 is the ASCII code for backspace
            if character == (8u8 as char) {
                name.pop();
            } else if character != ' ' {
                name.push(character);
            }
            if name.len() <= 20 {
                STATE.get().write().unwrap().name = String::from(name);
            }
        }
    }
}
