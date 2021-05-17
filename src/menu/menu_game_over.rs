use ggez::{
    graphics::{self, Text},
    nalgebra::Point2,
    Context,
};

use crate::{
    game::{BACKGROUND_COLOR, LIGHT_COLOR},
    piece::piece::PieceColor,
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::{clickable::ClickableGroup, menu_state::Menu};

pub(crate) const GAME_OVER_START_X: f32 = SCREEN_WIDTH / 2.0 - SCREEN_WIDTH * 0.4 / 2.0;
pub(crate) const GAME_OVER_START_Y: f32 = SCREEN_HEIGHT / 2.0 - SCREEN_HEIGHT * 0.7 / 2.0;
pub(crate) const GAME_OVER_MENU_WIDTH: f32 = SCREEN_WIDTH * 0.4;
pub(crate) const GAME_OVER_MENU_HEIGHT: f32 = SCREEN_HEIGHT * 0.7;

impl Menu {
    pub(crate) fn render_game_over(&mut self, ctx: &mut Context, winner: Option<PieceColor>) {
        // Draw list
        if let Ok(drawable) = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                GAME_OVER_START_X,
                GAME_OVER_START_Y,
                GAME_OVER_MENU_WIDTH,
                GAME_OVER_MENU_HEIGHT,
            ),
            graphics::Color::from(BACKGROUND_COLOR),
        ) {
            graphics::draw(ctx, &drawable, graphics::DrawParam::default())
                .expect("Could not draw list");
        }

        let mut text = Text::new("");
        match winner {
            Some(PieceColor::White) => {
                text = Text::new("White won");
            }
            Some(PieceColor::Black) => {
                text = Text::new("Black won");
            }
            None => {}
        }

        let scale = 50.0;
        text.set_font(self.font, graphics::Scale::uniform(scale));
        text.set_bounds(
            Point2::new(SCREEN_WIDTH * 0.4, 50.0),
            graphics::Align::Center,
        );

        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::default()
                .dest(Point2::<f32>::new(
                    SCREEN_WIDTH / 2.0 - GAME_OVER_MENU_WIDTH / 2.0,
                    SCREEN_HEIGHT / 2.0 - GAME_OVER_MENU_HEIGHT / 2.0 + 50.0,
                ))
                .color(graphics::Color::from(LIGHT_COLOR)),
        )
        .expect("Error drawing clickable text");

        // Draw clickables
        self.draw_clickables(ctx, vec![ClickableGroup::GameOverMenu]);
    }
}
