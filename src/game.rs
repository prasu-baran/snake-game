use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    show_restart_prompt: bool, // New
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            show_restart_prompt: false, 
        }
    }

   pub fn key_pressed(&mut self, key: Key) -> bool {
    if self.show_restart_prompt {
        match key {
    _ => {}
}

        return false; // Stay in prompt
    }

    if self.game_over {
        return false;
    }

    let dir = match key {
        Key::Up => Some(Direction::Up),
        Key::Down => Some(Direction::Down),
        Key::Left => Some(Direction::Left),
        Key::Right => Some(Direction::Right),
        _ => Some(self.snake.head_direction()),
    };

    if let Some(d) = dir {
        if d != self.snake.head_direction().opposite() {
            self.update_snake(Some(d));
        }
    }
    false
}


    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
    // Draw snake
    self.snake.draw(con, g);

    // Draw food
    if self.food_exists {
        draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
    }

    // Draw borders
    draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
    draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

    // Game over popup
    if self.show_restart_prompt {
        // Semi-transparent overlay
        draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);

        // Main game over text
        let text_transform = con.transform.trans(50.0, 200.0);
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
            .draw("Game Over!", glyphs, &con.draw_state, text_transform, g)
            .ok();

        // Instruction text
        let text_transform2 = con.transform.trans(50.0, 250.0);
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
            .draw("Press R to Restart or Q to Quit", glyphs, &con.draw_state, text_transform2, g)
            .ok();
    }
}



    pub fn update(&mut self, delta_time: f64) {
    if self.game_over {
        self.show_restart_prompt = true; // Enable popup
        return; // Stop game logic until player chooses
    }

    self.waiting_time += delta_time;

    if !self.food_exists {
        self.add_food();
    }

    if self.waiting_time > MOVING_PERIOD {
        self.update_snake(None);
    }
}


    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}