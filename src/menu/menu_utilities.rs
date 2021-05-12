use ggez::graphics;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{clickable::{Clickable, Transform}, menu_state::{LIST_HEIGHT, LIST_ITEM_HEIGHT, LIST_ITEM_MARGIN, LIST_ITEM_WIDTH, LIST_WIDTH, Menu}};

pub(crate) fn is_within_boundary(clickable: &Clickable, x: f32, y: f32, scroll: f32) -> bool {

	let scroll_addition = -1.0 * apply_scroll(scroll);

	if clickable.list_item {
		x > clickable.transform.x as f32 && x < (clickable.transform.x + clickable.transform.width) as f32 &&
		y + scroll_addition > clickable.transform.y as f32 && y + scroll_addition < (clickable.transform.y + clickable.transform.height) as f32
	} else {
		x > clickable.transform.x as f32 && x < (clickable.transform.x + clickable.transform.width) as f32 &&
		y > clickable.transform.y as f32 && y < (clickable.transform.y + clickable.transform.height) as f32
	}
}

pub(crate) fn apply_scroll(scroll: f32) -> f32 {
	return -1.0 * scroll * 25.0
}

pub(crate) fn reverse_scroll(scroll: f32) -> f32 {
	return scroll / 25.0;
}

/**
	This function converts relative coordinates
	(from the list point of view) to the actual
	screen coorindates.
*/
impl Menu {
	pub(crate) fn list_from_rel_to_real(x: f32, y: f32) -> (f32, f32) {
		let real_x = x - LIST_WIDTH / 2.0 + 3.0/4.0 * SCREEN_WIDTH;
		let real_y = y - LIST_HEIGHT / 2.0 + SCREEN_HEIGHT / 2.0;
		return (real_x, real_y);
	}

	pub(crate) fn list_from_real_to_rel(x: f32, y: f32) -> (f32, f32) {
		let rel_x = x + LIST_WIDTH / 2.0 - 3.0/4.0 * SCREEN_WIDTH;
		let rel_y = y + LIST_HEIGHT / 2.0 - SCREEN_HEIGHT / 2.0;
		return (rel_x, rel_y)
	}

	pub(crate) fn generate_list_item_from_list(elements: Vec<&str>) -> Vec<Clickable> {

		let mut clickables: Vec<Clickable> = Vec::new();

		let (x_pos, y_pos) = Menu::list_from_rel_to_real(
			LIST_WIDTH / 2.0 - LIST_ITEM_WIDTH / 2.0, 
			0.0
		);

		for i in 0..elements.len() {
			clickables.push(Clickable {
				transform: Transform {
					x: x_pos as i32,
					y: (y_pos + i as f32 * (LIST_ITEM_HEIGHT + LIST_ITEM_MARGIN)) as i32,
					height: LIST_ITEM_HEIGHT as i32,
					width: LIST_ITEM_WIDTH as i32
				},
				hovered: false,
				list_item: true,
				color: graphics::Color::from_rgb(100, 0, 200),
				text: graphics::Text::new(elements[i].clone())
			})
		}

		return clickables;
	}

	pub(crate) fn max_scroll(element_count: f32, bottom_margin: f32) -> f32 {
		let virtual_size = (LIST_ITEM_HEIGHT + bottom_margin) * element_count;
		return reverse_scroll(virtual_size - LIST_HEIGHT);
	}

	pub(crate) fn list_elements(&self) -> f32 {
		let mut elements = 0.0;
		for i in 0..self.clickables.len() {
			if self.clickables[i].list_item {
				elements += 1.0;
			}
		}
		return elements
	}
}