use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawMode, DrawParam, Image, MeshBuilder, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{
    event_handler::{BOARD_ORIGO_X, BOARD_ORIGO_Y, BOARD_SIZE, TILE_SIZE},
    game::Game,
    piece::piece::get_piece_rect,
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
            if index == *i {
                let rect = Rect::new(abs_x, abs_y, TILE_SIZE as f32, TILE_SIZE as f32);
                hidden_tiles.rectangle(
                    DrawMode::fill(),
                    rect,
                    graphics::Color::from_rgba(30, 30, 30, 240),
                );
                continue;
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
