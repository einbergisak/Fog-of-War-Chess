use ggez::{graphics};

#[derive(Clone)]
pub(crate) struct Clickable {
	pub(crate) x: i32,
	pub(crate) y: i32,
	pub(crate) width: i32,
	pub(crate) height: i32,
	pub(crate) color: graphics::Color,
	pub(crate) text: graphics::Text,
	pub(crate) hovered: bool
}