use super::clickable::Clickable;

pub(crate) fn is_within_boundary(clickable: &Clickable, x: f32, y: f32) -> bool {
	x > clickable.x as f32 && x < (clickable.x + clickable.width) as f32 &&
	y > clickable.y as f32 && y < (clickable.y + clickable.height) as f32
}