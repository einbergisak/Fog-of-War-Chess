use ggez::graphics;
use ggez::{
    event::{KeyCode, KeyMods},
    Context,
};

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

pub(crate) fn on_key_down(keycode: KeyCode, keymods: KeyMods, _repeat: bool) {
    let mut name = STATE.get().read().unwrap().name.clone();

    match keycode {
        KeyCode::Escape => {
            STATE.get().write().unwrap().name = String::from("");
        }
        KeyCode::Back => {
            name.pop();
        }
        KeyCode::A => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("A")
            } else {
                name.push_str("a")
            };
        }
        KeyCode::B => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("B")
            } else {
                name.push_str("b")
            };
        }
        KeyCode::C => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("C")
            } else {
                name.push_str("c")
            };
        }
        KeyCode::D => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("D")
            } else {
                name.push_str("d")
            };
        }
        KeyCode::E => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("E")
            } else {
                name.push_str("e")
            };
        }
        KeyCode::F => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("F")
            } else {
                name.push_str("f")
            };
        }
        KeyCode::G => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("G")
            } else {
                name.push_str("g")
            };
        }
        KeyCode::H => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("H")
            } else {
                name.push_str("h")
            };
        }
        KeyCode::I => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("I")
            } else {
                name.push_str("i")
            };
        }
        KeyCode::J => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("J")
            } else {
                name.push_str("j")
            };
        }
        KeyCode::K => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("K")
            } else {
                name.push_str("k")
            };
        }
        KeyCode::L => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("L")
            } else {
                name.push_str("l")
            };
        }
        KeyCode::M => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("M")
            } else {
                name.push_str("m")
            };
        }
        KeyCode::N => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("N")
            } else {
                name.push_str("n")
            };
        }
        KeyCode::O => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("O")
            } else {
                name.push_str("o")
            };
        }
        KeyCode::P => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("P")
            } else {
                name.push_str("p")
            };
        }
        KeyCode::Q => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("Q")
            } else {
                name.push_str("q")
            };
        }
        KeyCode::R => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("R")
            } else {
                name.push_str("r")
            };
        }
        KeyCode::S => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("S")
            } else {
                name.push_str("s")
            };
        }
        KeyCode::T => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("T")
            } else {
                name.push_str("t")
            };
        }
        KeyCode::U => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("U")
            } else {
                name.push_str("u")
            };
        }
        KeyCode::V => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("V")
            } else {
                name.push_str("v")
            };
        }
        KeyCode::W => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("W")
            } else {
                name.push_str("w")
            };
        }
        KeyCode::X => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("X")
            } else {
                name.push_str("x")
            };
        }
        KeyCode::Y => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("Y")
            } else {
                name.push_str("y")
            };
        }
        KeyCode::Z => {
            if keymods.eq(&KeyMods::SHIFT) {
                name.push_str("Z")
            } else {
                name.push_str("z")
            };
        }
        _ => {}
    }

    if name.len() <= 20 {
        STATE.get().write().unwrap().name = String::from(name);
    }
}
