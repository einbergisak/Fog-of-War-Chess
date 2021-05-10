use ggez::{Context, graphics};

use super::{clickable::Clickable, menu_utilities::is_within_boundary};

pub(crate) struct Menu {
	pub(crate) visible: bool,
	pub(crate) clickables: Vec<Clickable>,
	last_iteration_hover: bool
}

impl Menu {
	pub(crate) fn new() -> Menu {
		Menu {
			visible: true,
			clickables: Vec::new(),
			last_iteration_hover: false
		}
	}

	pub(crate) fn on_mouse_move(&mut self, ctx: &mut Context, x: f32, y: f32) {
		let mut is_hovering = false;
		for i in 0..self.clickables.len() {
			let result = is_within_boundary(&self.clickables[i], x, y);
			self.clickables[i].hovered = result;
			if result {
				is_hovering = true;
			}
		}
		// We only want to change the cursor state if it
		// has actually changed, no unneccessary changes
		if is_hovering != self.last_iteration_hover {
			if is_hovering {
				ggez::input::mouse::set_cursor_grabbed(ctx, true).expect("Cursor entering clickable failed");
				ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Hand)
			} else {
				ggez::input::mouse::set_cursor_grabbed(ctx, false).expect("Cursor leaving clickable failed");
				ggez::input::mouse::set_cursor_type(ctx, ggez::input::mouse::MouseCursor::Default);
			}
		}
		self.last_iteration_hover = is_hovering;
	}

	pub(crate) fn render(&self, ctx: &mut Context) {
		let clickables = &self.clickables;

		for i in 0..clickables.len() {

			let mut color = clickables[i].color;
			if clickables[i].hovered {
				color = graphics::Color::from_rgb_u32(clickables[i].color.to_rgb_u32() - 5000);
			}

			let clickable = graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(), 
				graphics::Rect::new(
					clickables[i].x as f32, 
					clickables[i].y as f32, 
					clickables[i].width as f32, 
					clickables[i].height as f32
				), 
				color
			);

			match clickable {
				Ok(drawable_clickable) => {
					// Optimization here, draw everything at once (Isak help me here :D)
					graphics::draw(ctx, &drawable_clickable, graphics::DrawParam::default());
				}
				Err(_) => {}
			}
		}
	}
}