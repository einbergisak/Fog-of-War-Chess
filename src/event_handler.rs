use ggez::{Context, GameResult, event::{EventHandler, MouseButton}, graphics::{self, DrawParam, Font, Image, Text, spritebatch::SpriteBatch}, nalgebra::Point2};

use crate::{Game, SCREEN_HEIGHT, SCREEN_WIDTH, STATE, game::{BACKGROUND_COLOR, LIGHT_COLOR}, piece::{get_piece_rect, get_valid_move_indices, Piece}, render_utilities::{flip_board, flip_index, translate_to_index}};

use ggez::{
    graphics::{DrawMode, Mesh, MeshBuilder, Rect},
    input::mouse,
    nalgebra::Vector2,
};

use crate::{
    move_struct::{Move, MoveType::*},
    piece::{PieceColor::*, PieceType::*},
    render_utilities::translate_to_coords,
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

                //STATE.get().write().unwrap().event_validation.create_room = false;
            } else if event_validation.join_room {
                self.menu.visible = false;

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
                    .dest(Point2::<f32>::new(
                        0.0,
                        0.0
                    ))
                    .color(graphics::Color::from(LIGHT_COLOR))
            ).expect("Error drawing clickable text");
        }

        let piece_image = Image::new(ctx, "/pieces.png")?;
        let mut piece_batch = SpriteBatch::new(piece_image.clone());

        let board_to_render = if self.playing_as_white {
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
        for (index, tile) in board_to_render.iter().enumerate() {
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

        // Draw pieces
        graphics::draw(ctx, &piece_batch, (Point2::<f32>::new(0.0, 0.0),))?;

        // Pawn promotion menu
        if let Some(Move {
            piece,
            piece_dest_index,
            captured_piece: _,
            move_type: Promotion(_),
        }) = self.promoting_pawn.as_ref()
        {
            let bounds = Rect::new_i32(BOARD_ORIGO_X as i32, BOARD_ORIGO_Y as i32, BOARD_WIDTH, BOARD_WIDTH);
            let overlay = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                bounds,
                graphics::Color::from_rgba(240, 240, 240, 40),
            )
            .unwrap();
            let flipped_x_index = if self.playing_as_white {
                translate_to_coords(flip_index(*piece_dest_index)).0
            } else {
                translate_to_coords(*piece_dest_index).0
            };

            let mut promotion_prompt = MeshBuilder::new();

            let image_y = if let White = piece.color { 0.0 } else { 0.5 };
            let mut promotion_piece_batch = SpriteBatch::new(piece_image);
            for n in 1..=4 {
                let src_rect = Rect::new(n as f32 / 6.0, image_y, 1.0 / 6.0, 0.5);
                let (x, y) = ((flipped_x_index as i32) * TILE_SIZE, (n - 1) * TILE_SIZE);
                let mut dest_rect = Rect::new_i32(x + BOARD_ORIGO_X as i32, y + BOARD_ORIGO_Y as i32, TILE_SIZE, TILE_SIZE);
                let center = Point2::new(
                    dest_rect.x + dest_rect.w / 2.0,
                    dest_rect.y + dest_rect.w / 2.0,
                );
                let scale = if dest_rect.contains(mouse::position(ctx)) {
                    promotion_prompt.rectangle(
                        DrawMode::fill(),
                        dest_rect,
                        graphics::Color::from_rgb(191, 43, 33),
                    );
                    Vector2::new(1.0, 1.0)
                } else {
                    let mut center_rect = dest_rect.clone();
                    center_rect.move_to(center);
                    promotion_prompt.circle(
                        DrawMode::fill(),
                        center_rect.point(),
                        (TILE_SIZE / 2) as f32,
                        1.0,
                        graphics::Color::from_rgb(214, 214, 214),
                    );

                    dest_rect.translate(Vector2::new(15.0, 15.0));
                    Vector2::new(0.7, 0.7)
                };
                promotion_piece_batch.add(
                    DrawParam::default()
                        .src(src_rect)
                        .dest(dest_rect.point())
                        .scale(scale),
                );
            }
            let promotion_prompt = promotion_prompt.build(ctx)?;

            graphics::draw(ctx, &overlay, (Point2::<f32>::new(0.0, 0.0),))?;
            graphics::draw(ctx, &promotion_prompt, (Point2::<f32>::new(0.0, 0.0),))?;
            graphics::draw(ctx, &promotion_piece_batch, (Point2::<f32>::new(0.0, 0.0),))?;
        }

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

                // Calculates (on screen) list index (if cursor is in bounds) of the clicked tile
                let x_tile = ((x - start_x) / TILE_SIZE as f32) as usize;
                let y_tile = ((y - start_y) / TILE_SIZE as f32) as usize;

                let mut index = translate_to_index(x_tile, y_tile);
                if self.playing_as_white {
                    index = flip_index(index);
                }

                // Pawn promotion interface
                if let Some(Move {
                    piece,
                    piece_dest_index,
                    captured_piece: _,
                    move_type: Promotion(_),
                }) = self.promoting_pawn.take()
                {
                    let piece_dest_index = piece_dest_index.to_owned();
                    let (promotion_x, _promotion_y) = if self.playing_as_white {
                        translate_to_coords(flip_index(piece_dest_index))
                    } else {
                        translate_to_coords(piece_dest_index)
                    };

                    // If clicking a tile within the promotion interface: promote to the chosen piece
                    if x_tile == promotion_x && y_tile <= 3 {
                        let piece_type = match y_tile {
                            0 => Queen,
                            1 => Bishop,
                            2 => Knight,
                            3 => Rook(true),
                            _ => panic!("Promotion out of bounds error. This shouldn't happen."),
                        };
                        let captured_piece = self.board[piece_dest_index].take();
                        self.board[piece_dest_index] = Some(Piece {
                            piece_type,
                            color: piece.color,
                            index: piece_dest_index,
                        });
                        let move_ = Move {
                            piece: piece,
                            piece_dest_index: piece_dest_index,
                            captured_piece,
                            move_type: Promotion(piece_type),
                        };
                        self.move_history.push(move_);
                        self.connection.send("opponent", &move_.to_string());
                        // Your turn is over once you've made a move
                        self.active_turn = !self.active_turn;
                    }
                    // If clicking outside the promotion interface: return the pawn to its source position.
                    else {
                        let index = piece.get_index();
                        self.board[index] = Some(piece);
                    }
                }

                // Attempts to grab a piece from the given tile
                if let Some(piece) = self.board[index].take() {
                    match &piece.color {
                        crate::piece::PieceColor::White if !self.playing_as_white => {
                            self.board[index] = Some(piece);
                            return;
                        }
                        crate::piece::PieceColor::Black if self.playing_as_white => {
                            self.board[index] = Some(piece);
                            return;
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
                    return;
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

                let mut piece_source_index =
                    translate_to_index(source_x as usize, source_y as usize);
                let mut piece_dest_index = translate_to_index(x_tile, y_tile);

                if self.playing_as_white {
                    piece_dest_index = flip_index(piece_dest_index);
                    piece_source_index = flip_index(piece_source_index);
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

                    // En passant
                    println!("Moving piece: {:?}", &piece);
                    if piece.piece_type == Pawn(true)
                        && ((piece.color == White
                            && translate_to_coords(piece_dest_index).1 == BOARD_SIZE - 1)
                            || (piece.color == Black
                                && translate_to_coords(piece_dest_index).1 == 0))
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
