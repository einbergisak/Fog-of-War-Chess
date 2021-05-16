use crate::piece::piece::Piece;
use crate::render_utilities::flip_index;
use crate::Game;

use ggez::{
    graphics::{
        self, spritebatch::SpriteBatch, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect,
    },
    input::mouse,
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

use crate::{
    event_handler::{BOARD_ORIGO_X, BOARD_ORIGO_Y, BOARD_WIDTH, TILE_SIZE},
    move_struct::{Move, MoveType},
    piece::piece::PieceColor,
    render_utilities::translate_to_coords,
};

use crate::{move_struct::MoveType::*, piece::piece::PieceType::*};

pub(crate) fn check_promotion(game: &mut Game, x_tile: usize, y_tile: usize) {
    if let Some(Move {
        piece,
        piece_dest_index,
        captured_piece: _,
        move_type: Promotion(_),
    }) = game.promoting_pawn.take()
    {
        let piece_dest_index = piece_dest_index.to_owned();
        let (promotion_x, _promotion_y) = if game.playing_as_white {
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
            let captured_piece = game.board[piece_dest_index].take();
            game.board[piece_dest_index] = Some(Piece {
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
            game.move_history.push(move_);
            game.connection.send("opponent", &move_.to_string());
            // Your turn is over once you've made a move
            game.active_turn = !game.active_turn;
        }
        // If clicking outside the promotion interface: return the pawn to its source position.
        else {
            let index = piece.get_index();
            game.board[index] = Some(piece);
        }
    }
}

pub(crate) fn render_promotion_interface(game: &Game, ctx: &mut Context) -> GameResult<()> {
    // Pawn promotion menu
    if let Some(Move {
        piece,
        piece_dest_index,
        captured_piece: _,
        move_type: MoveType::Promotion(_),
    }) = game.promoting_pawn.as_ref()
    {
        let piece_image = Image::new(ctx, "/pieces.png")?;
        let bounds = Rect::new_i32(
            BOARD_ORIGO_X as i32,
            BOARD_ORIGO_Y as i32,
            BOARD_WIDTH,
            BOARD_WIDTH,
        );
        let overlay = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            bounds,
            graphics::Color::from_rgba(240, 240, 240, 40),
        )
        .unwrap();
        let flipped_x_index = if game.playing_as_white {
            translate_to_coords(flip_index(*piece_dest_index)).0
        } else {
            translate_to_coords(*piece_dest_index).0
        };

        let mut promotion_prompt = MeshBuilder::new();

        let image_y = if let PieceColor::White = piece.color {
            0.0
        } else {
            0.5
        };
        let mut promotion_piece_batch = SpriteBatch::new(piece_image);
        for n in 1..=4 {
            let src_rect = Rect::new(n as f32 / 6.0, image_y, 1.0 / 6.0, 0.5);
            let (x, y) = ((flipped_x_index as i32) * TILE_SIZE, (n - 1) * TILE_SIZE);
            let mut dest_rect = Rect::new_i32(
                x + BOARD_ORIGO_X as i32,
                y + BOARD_ORIGO_Y as i32,
                TILE_SIZE,
                TILE_SIZE,
            );
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
        graphics::draw(ctx, &promotion_piece_batch, (Point2::<f32>::new(0.0, 0.0),))
    } else {
        Ok(())
    }
}
