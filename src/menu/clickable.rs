use ggez::{graphics};

pub(crate) enum ClickableId {
	CreateGameButton = 0,
	ListItem = 1
}
pub(crate) struct Transform {
	pub(crate) x: i32,
	pub(crate) y: i32,
	pub(crate) width: i32,
	pub(crate) height: i32,
}
pub(crate) struct Clickable {
	pub(crate) id: ClickableId,
	pub(crate) transform: Transform,
	pub(crate) color: graphics::Color,
	pub(crate) text: graphics::Text,
	pub(crate) hovered: bool,
	pub(crate) list_item: bool
}