use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawMode, DrawParam, Image, MeshBuilder, Rect},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

use crate::{
    event_handler::{BOARD_ORIGO_X, BOARD_ORIGO_Y, BOARD_SIZE, TILE_SIZE},
    game::Game,
    piece::piece::{get_piece_rect, get_valid_move_indices},
};

pub(crate) fn flip_index(index: usize) -> usize {
    let list_size = BOARD_SIZE * BOARD_SIZE;
    return list_size - index - 1;
}

pub(crate) fn flip_pos(pos: (usize, usize)) -> (usize, usize) {
    let x = BOARD_SIZE - 1 - pos.0;
    let y = BOARD_SIZE - 1 - pos.1;
    return (x, y);
}

/// Translates from game coordinates to list index
pub(crate) fn translate_to_index(x: usize, y: usize) -> usize {
    return y * BOARD_SIZE + x;
}

/// Translates from list index to game coordinates
pub(crate) fn translate_to_coords(index: usize) -> (usize, usize) {
    let y = index / 8;
    let x = index % 8;
    return (x, y);
}

pub(crate) fn render_fog_and_pieces(game: &Game, ctx: &mut Context) -> GameResult<()> {
    let piece_image = Image::new(ctx, "/pieces.png")?;
    let mut piece_batch = SpriteBatch::new(piece_image);

    let grabbed_index: Option<usize>;
    let grabbed_param: Option<DrawParam>;

    // Renders the grabbed piece
    if let Some(piece) = &game.grabbed_piece {
        let (x, y) = if game.playing_as_white {
            flip_pos(piece.get_pos())
        } else {
            piece.get_pos()
        };
        grabbed_index = Some(translate_to_index(x.clone(), y.clone()));

        let rect = get_piece_rect(&piece);
        let (cursor_x, cursor_y) = (
            ggez::input::mouse::position(ctx).x,
            ggez::input::mouse::position(ctx).y,
        );
        grabbed_param = Some(DrawParam::default().src(rect).dest(Point2::new(
            cursor_x - TILE_SIZE as f32 / 2.0,
            cursor_y - TILE_SIZE as f32 / 2.0,
        )));

        // Renders a "ghost image" of the grabbed piece at its source location
        let flipped_index = if game.playing_as_white {
            flip_index(piece.get_index())
        } else {
            piece.get_index()
        };

        let rel_x = (flipped_index % BOARD_SIZE) as f32;
        let rel_y = (flipped_index / BOARD_SIZE) as f32;
        let abs_x = rel_x * TILE_SIZE as f32 + BOARD_ORIGO_X;
        let abs_y = rel_y * TILE_SIZE as f32 + BOARD_ORIGO_Y;
        let rect = get_piece_rect(piece);
        let param = DrawParam::default()
            .src(rect)
            .dest(Point2::new(abs_x, abs_y))
            .color(graphics::Color::from_rgba(100, 100, 100, 100));

        piece_batch.add(param);
    } else {
        grabbed_index = None;
        grabbed_param = None;
    };

    let mut hidden_tiles = MeshBuilder::new();

    // Render each piece in the board
    for (index, tile) in game.board.iter().enumerate() {
        let flipped_index = if game.playing_as_white {
            flip_index(index)
        } else {
            index
        };

        let rel_x = (flipped_index % BOARD_SIZE) as f32;
        let rel_y = (flipped_index / BOARD_SIZE) as f32;
        let abs_x = rel_x * TILE_SIZE as f32 + BOARD_ORIGO_X;
        let abs_y = rel_y * TILE_SIZE as f32 + BOARD_ORIGO_Y;

        if let Some(i) = &grabbed_index {
            if game.playing_as_white {
                if index == flip_index(*i) {
                    continue;
                }
            } else {
                if index == *i {
                    continue;
                }
            }
        }
        // Only draw the tiles which are in your vision
        if game.available_moves.contains(&index) {
            if let Some(piece) = tile {
                let rect = get_piece_rect(piece);
                let param = DrawParam::default()
                    .src(rect)
                    .dest(Point2::new(abs_x, abs_y));

                piece_batch.add(param);
            }
        }
        // The other tiles are hidden in the fog of war
        else {
            let rect = Rect::new(abs_x, abs_y, TILE_SIZE as f32, TILE_SIZE as f32);
            hidden_tiles.rectangle(
                DrawMode::fill(),
                rect,
                graphics::Color::from_rgba(30, 30, 30, 240),
            );
        }
    }

    // Draw hidden tiles (aka "fog")
    let hidden_tiles_mesh = hidden_tiles.build(ctx)?;
    graphics::draw(ctx, &hidden_tiles_mesh, (Point2::<f32>::new(0.0, 0.0),))?;

    if let Some(param) = grabbed_param {
        piece_batch.add(param);
    }

    // Draw pieces
    graphics::draw(ctx, &piece_batch, (Point2::<f32>::new(0.0, 0.0),))
}

/// Renders highlighting for your available moves and for the prievious move (if it was visible to you)
pub(crate) fn render_movement_indication(game: &Game, ctx: &mut Context) -> GameResult<()> {
    let mut movement_indication_batch = SpriteBatch::new(Image::new(ctx, "/markers.png")?);
    for piece in [game.grabbed_piece, game.selected_piece].iter() {
        if let Some(piece) = piece {
            // Adds movement indication dots and highlighting
            for index in get_valid_move_indices(game, piece) {
                let mut dp = DrawParam::default().offset(Point2::new(0.5, 0.5)).dest({
                    let (x, y) = if game.playing_as_white {
                        flip_pos(translate_to_coords(index))
                    } else {
                        translate_to_coords(index)
                    };
                    let x_pos =
                        x as f32 * TILE_SIZE as f32 + BOARD_ORIGO_X + (TILE_SIZE / 2) as f32;
                    let y_pos =
                        y as f32 * TILE_SIZE as f32 + BOARD_ORIGO_Y + (TILE_SIZE / 2) as f32;
                    Point2::new(x_pos, y_pos)
                });
                if game.board[index].is_some() {
                    dp.src = Rect::new(1.0 / 4.0, 0.0, 1.0 / 4.0, 1.0)
                } else {
                    dp = dp.scale(Vector2::new(0.3, 0.3));
                    dp.src = Rect::new(0.0 / 4.0, 0.0, 1.0 / 4.0, 1.0)
                }
                movement_indication_batch.add(dp);
            }
        }
    }

    // Highlights the source- and destination tile of the previous move (if the moves are visible to you)
    if let Some(m) = game.move_history.last() {
        // Source tile
        if game.available_moves.contains(&m.piece.index) {
            let dp_source_tile = DrawParam::default()
                .src(Rect::new(3.0 / 4.0, 0.0, 1.0 / 4.0, 1.0))
                .dest({
                    let (x, y) = if game.playing_as_white {
                        flip_pos(translate_to_coords(m.piece.index))
                    } else {
                        translate_to_coords(m.piece.index)
                    };
                    let x_pos = x as f32 * TILE_SIZE as f32 + BOARD_ORIGO_X;
                    let y_pos = y as f32 * TILE_SIZE as f32 + BOARD_ORIGO_Y;
                    Point2::new(x_pos, y_pos)
                });
            movement_indication_batch.add(dp_source_tile);
        }

        // Destination tile
        if game.available_moves.contains(&m.piece_dest_index) {
            let dp_dest_tile = DrawParam::default()
                .src(Rect::new(3.0 / 4.0, 0.0, 1.0 / 4.0, 1.0))
                .dest({
                    let (x, y) = if game.playing_as_white {
                        flip_pos(translate_to_coords(m.piece_dest_index))
                    } else {
                        translate_to_coords(m.piece_dest_index)
                    };
                    let x_pos = x as f32 * TILE_SIZE as f32 + BOARD_ORIGO_X;
                    let y_pos = y as f32 * TILE_SIZE as f32 + BOARD_ORIGO_Y;
                    Point2::new(x_pos, y_pos)
                });
            movement_indication_batch.add(dp_dest_tile);
        }
    }

    // Renders premoves
    if let Some((piece, piece_dest_index)) = game.premove {
        // Source tile
        let dp_source_tile = DrawParam::default()
            .src(Rect::new(2.0 / 4.0, 0.0, 1.0 / 4.0, 1.0))
            .dest({
                let (x, y) = if game.playing_as_white {
                    flip_pos(translate_to_coords(piece.index))
                } else {
                    translate_to_coords(piece.index)
                };
                let x_pos = x as f32 * TILE_SIZE as f32 + BOARD_ORIGO_X;
                let y_pos = y as f32 * TILE_SIZE as f32 + BOARD_ORIGO_Y;
                Point2::new(x_pos, y_pos)
            });
        movement_indication_batch.add(dp_source_tile);

        // Destination tile
        let dp_dest_tile = DrawParam::default()
            .src(Rect::new(2.0 / 4.0, 0.0, 1.0 / 4.0, 1.0))
            .dest({
                let (x, y) = if game.playing_as_white {
                    flip_pos(translate_to_coords(piece_dest_index))
                } else {
                    translate_to_coords(piece_dest_index)
                };
                let x_pos = x as f32 * TILE_SIZE as f32 + BOARD_ORIGO_X;
                let y_pos = y as f32 * TILE_SIZE as f32 + BOARD_ORIGO_Y;
                Point2::new(x_pos, y_pos)
            });
        movement_indication_batch.add(dp_dest_tile);
    }

    // Highlights the square underneath your grabbed piece to indicate where it will be placed if dropped.
    if let Some(piece) = &game.grabbed_piece {
        let (cursor_x, cursor_y) = (
            ggez::input::mouse::position(ctx).x,
            ggez::input::mouse::position(ctx).y,
        );

        let x_tile = ((cursor_x - BOARD_ORIGO_X) / TILE_SIZE as f32) as usize;
        let y_tile = ((cursor_y - BOARD_ORIGO_Y) / TILE_SIZE as f32) as usize;

        let mut hovered_index = translate_to_index(x_tile, y_tile);

        if game.playing_as_white {
            hovered_index = flip_index(hovered_index);
        }

        // Only highlights the square if it is a valid move
        if get_valid_move_indices(game, piece).contains(&hovered_index) {
            let dest_rect = Point2::new(
                x_tile as f32 * TILE_SIZE as f32 + BOARD_ORIGO_X,
                y_tile as f32 * TILE_SIZE as f32 + BOARD_ORIGO_Y,
            );
            let src_rect = Rect::new(3.0 / 4.0, 0.0, 1.0 / 4.0, 1.0);
            let dp = DrawParam::default()
                .src(src_rect)
                .dest(dest_rect)
                .color(graphics::Color::from_rgba(100, 200, 100, 250));
            movement_indication_batch.add(dp);
        }
    }

    graphics::draw(
        ctx,
        &movement_indication_batch,
        (Point2::<f32>::new(0.0, 0.0),),
    )
}
