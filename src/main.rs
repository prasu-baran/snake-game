extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;

use piston_window::{*, types::Color};
use crate::game::Game;
use crate::draw::to_coord_u32;

// ===== Configurable Constants =====
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const BOARD_WIDTH: i32 = 30;
const BOARD_HEIGHT: i32 = 30;
const UPDATES_PER_SECOND: u64 = 10; // Snake speed

// ===== Window Setup Function =====
fn create_window(width: i32, height: i32) -> PistonWindow {
    WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .expect("Failed to create window")
}

fn main() {
    let mut window = create_window(BOARD_WIDTH, BOARD_HEIGHT);
    window.set_ups(UPDATES_PER_SECOND);

    // Load font for popup text
    let font_path = "FiraSans-Regular.ttf"; // Make sure this is next to your main.rs or exe
   let mut glyphs = window.load_font(font_path).expect("Failed to load font");
    let mut game = Game::new(BOARD_WIDTH, BOARD_HEIGHT);

    while let Some(event) = window.next() {
        // Handle key presses
        if let Some(Button::Keyboard(key)) = event.press_args() {
           match key {
    Key::Up | Key::Down | Key::Left | Key::Right => { game.key_pressed(key); }
    Key::R => { game.restart(); }
    _ => {}
}

        }

        // Render only when a render event occurs
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |c, g, device| {
                clear(BACK_COLOR, g);
                game.draw(&c, g, &mut glyphs); // pass glyphs to draw text
                glyphs.factory.encoder.flush(device); // flush text rendering
            });
        }

        // Update game state
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
