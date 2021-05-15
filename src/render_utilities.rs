use crate::{event_handler::BOARD_SIZE};

pub(crate) fn flip_index(index: usize) -> usize {
    let list_size = BOARD_SIZE * BOARD_SIZE;
    return list_size - index - 1;
}

/// Translates from game coordinates to list index
pub(crate) fn translate_to_index(x: usize, y: usize) -> usize {
    return y * BOARD_SIZE + x;
}

/// Translates from list index to game coordinates
pub(crate) fn translate_to_coords(index: usize) -> (usize, usize) {
    let y = index / 8;
    let x = index % 8;
    return (x, y);
}
