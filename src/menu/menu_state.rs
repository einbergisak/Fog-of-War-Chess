use ggez::{Context, graphics::{self}};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, menu::{clickable, menu_utilities::reverse_scroll}};
use super::{clickable::{Clickable, Transform}, menu_utilities::{apply_scroll, is_within_boundary}};

pub(crate) const LIST_WIDTH: f32 = SCREEN_WIDTH / 2.0 * 0.8;
pub(crate) const LIST_HEIGHT: f32 = SCREEN_HEIGHT as f32 * 0.8;
pub(crate) const LIST_START_X: f32 = ((( 3.0 * SCREEN_WIDTH as f32 / 4.0) - (LIST_WIDTH / 2.0)) as i32) as f32;
pub(crate) const LIST_START_Y: f32 = (SCREEN_HEIGHT as f32 / 2.0) - (LIST_HEIGHT / 2.0);
pub(crate) const LIST_ITEM_WIDTH: f32 = LIST_WIDTH * 0.8;
pub(crate) const LIST_ITEM_HEIGHT: f32 = 100.0;
pub(crate) const LIST_ITEM_MARGIN: f32 = 20.0;

pub(crate) struct List {
	transform: Transform,
	scroll: f32
}

pub(crate) struct Menu {
	pub(crate) visible: bool,
	pub(crate) clickables: Vec<Clickable>,
	pub(crate) list: List,
	last_iteration_hover: bool
}

impl Menu {
	pub(crate) fn new() -> Menu {
		Menu {
			visible: true,
			clickables: Menu::generate_list_item_from_list(vec![
				"First",
				"ping",
				"pong",
				"hello",
				"test",
				"TEST",
				"TEST2",
				"test3"
			]),
			last_iteration_hover: false,
			list: List {
				transform: Transform {
					x: LIST_START_X as i32,
					y: LIST_START_Y as i32,
					width: LIST_WIDTH as i32,
					height: LIST_HEIGHT as i32
				},
				scroll: 0.0
			}
		}
	}

	pub(crate) fn on_mouse_move(&mut self, ctx: &mut Context, x: f32, y: f32) {
		let mut is_hovering = false;
		for i in 0..self.clickables.len() {
			let result = is_within_boundary(&self.clickables[i], x, y, self.list.scroll);
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

	pub(crate) fn on_mouse_wheel(&mut self, _ctx: &mut Context, y: f32) {

		let mut last_list_clickable: Option<&Clickable> = None;
		for i in 0..self.clickables.len() {
			if self.clickables[i].list_item {
				last_list_clickable = Some(&self.clickables[i]);
			}
		}

		self.list.scroll -= y;

		if y < 0.0 {
			match last_list_clickable {
				Some(_) => {
					if self.list.scroll > Menu::max_scroll(self.list_elements(), LIST_ITEM_MARGIN) {
						self.list.scroll = Menu::max_scroll(self.list_elements(), LIST_ITEM_MARGIN);
					}
				}
				None => {}
			}
		}

		// Can never scroll over the first element
		if self.list.scroll < 0.0 {
			self.list.scroll = 0.0;
		}
	}

	pub(crate) fn render(&self, ctx: &mut Context) {

		// Draw list
		let list_drawable = graphics::Mesh::new_rectangle(
			ctx, 
			graphics::DrawMode::fill(), 
			graphics::Rect::new(
				self.list.transform.x as f32, 
				self.list.transform.y as f32, 
				self.list.transform.width as f32, 
				self.list.transform.height as f32
			), 
			graphics::Color::from_rgb(100, 100, 0)
		).expect("Could not render list");

		graphics::draw(ctx, &list_drawable, graphics::DrawParam::default()).expect("Could not draw list");

		let clickables = &self.clickables;

		// Go through all clickables and draw them
		for i in 0..clickables.len() {

			let mut color = clickables[i].color;
			if clickables[i].hovered {
				color = graphics::Color::from_rgb_u32(clickables[i].color.to_rgb_u32() - 5000);
			}

			// If the clickable is not a 
			// list item then we don't
			// scroll it
			let mut scroll = 0.0;
			if clickables[i].list_item {
				scroll = self.list.scroll
			}

			let clickable = graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(), 
				graphics::Rect::new(
					clickables[i].transform.x as f32, 
					clickables[i].transform.y as f32 + apply_scroll(scroll), 
					clickables[i].transform.width as f32, 
					clickables[i].transform.height as f32
				), 
				color
			);

			match clickable {
				Ok(drawable_clickable) => {
					// Optimization here, draw everything at once (Isak help me here :D)
					graphics::draw(ctx, &drawable_clickable, graphics::DrawParam::default()).expect("Could not draw clickable");
				}
				Err(_) => {}
			}
		}

		// Render overlappers
		let high_overlapper = graphics::Mesh::new_rectangle(
			ctx, 
			graphics::DrawMode::fill(), 
			graphics::Rect::new(
				LIST_START_X, 
				0.0, 
				LIST_WIDTH, 
				(SCREEN_HEIGHT - LIST_HEIGHT) / 2.0
			), 
			graphics::Color::from_rgb(0, 0, 0)
		);
		match high_overlapper {
		    Ok(overlapper) => {
				graphics::draw(ctx, &overlapper, graphics::DrawParam::default()).expect("This is a test");
			}
		    Err(_) => {}
		}

		let low_overlapper = graphics::Mesh::new_rectangle(
			ctx, 
			graphics::DrawMode::fill(), 
			graphics::Rect::new(
				LIST_START_X, 
				LIST_START_Y + LIST_HEIGHT, 
				LIST_WIDTH, 
				(SCREEN_HEIGHT - LIST_HEIGHT) / 2.0
			), 
			graphics::Color::from_rgb(0, 0, 0)
		);
		match low_overlapper {
		    Ok(overlapper) => {
				graphics::draw(ctx, &overlapper, graphics::DrawParam::default()).expect("This is a test");
			}
		    Err(_) => {}
		}
	}
}