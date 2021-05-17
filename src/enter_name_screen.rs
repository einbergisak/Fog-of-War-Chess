use ggez::graphics;
use ggez::Context;

use crate::{game::LIGHT_COLOR, menu::menu_state::Menu, SCREEN_HEIGHT, SCREEN_WIDTH, STATE};

impl Menu {
    pub(crate) fn render_name_interface(&mut self, ctx: &mut Context) {
        // Draw screen title
        self.draw_text(
            ctx,
            String::from("Enter your name"),
            (0.0, SCREEN_HEIGHT * 0.1),
            (SCREEN_WIDTH, SCREEN_HEIGHT * 0.1),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Center,
        );

        let name = &STATE.get().read().unwrap().name;
        const WRITING_WIDTH: f32 = 525.0;

        // Draw player name
        self.draw_text(
            ctx,
            format!("Name: {}", &name[..]),
            (
                SCREEN_WIDTH / 2.0 - WRITING_WIDTH / 2.0,
                SCREEN_HEIGHT / 3.0 - SCREEN_HEIGHT * 0.05 / 2.0,
            ),
            (WRITING_WIDTH, SCREEN_HEIGHT * 0.05),
            graphics::Color::from(LIGHT_COLOR),
            graphics::Align::Left,
        );

        // Draw text underline
        match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                SCREEN_WIDTH / 2.0 - WRITING_WIDTH / 2.0,
                SCREEN_HEIGHT / 3.0 + SCREEN_HEIGHT * 0.05 / 2.0 + 10.0,
                WRITING_WIDTH,
                2.0,
            ),
            graphics::Color::from(LIGHT_COLOR),
        ) {
            Ok(rect) => {
                graphics::draw(ctx, &rect, graphics::DrawParam::default())
                    .expect("Could not draw underline");
            }
            Err(_) => {}
        }
    }
}
