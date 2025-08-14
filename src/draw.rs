use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

pub const BLOCK_SIZE: f64 = 25.0;
pub const BLOCK_SIZE_U32: u32 = BLOCK_SIZE as u32;

/// Convert game grid coordinate to pixel coordinate
#[inline]
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// Convert game grid coordinate to pixel coordinate (u32)
#[inline]
pub fn to_coord_u32(game_coord: i32) -> u32 {
    (game_coord as u32) * BLOCK_SIZE_U32
}

/// Draw a single block (cell) on the screen
#[inline]
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = (x as f64) * BLOCK_SIZE;
    let gui_y = (y as f64) * BLOCK_SIZE;

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

/// Draw a rectangle of multiple blocks
#[inline]
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = (x as f64) * BLOCK_SIZE;
    let gui_y = (y as f64) * BLOCK_SIZE;
    let w = (width as f64) * BLOCK_SIZE;
    let h = (height as f64) * BLOCK_SIZE;

    rectangle(
        color,
        [gui_x, gui_y, w, h],
        con.transform,
        g,
    );
}
