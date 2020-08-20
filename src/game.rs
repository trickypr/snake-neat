extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs};

use crate::apple::*;
use crate::snake::*;
use crate::utils::*;

pub struct Game {
    pub snake: Snake,
    update_lock: bool,
    pub apple: Apple,
    pub score: u32,
    respawn: bool,
    pub moves: u32,
}

impl Game {
    pub fn new(snake: Snake) -> Game {
        Game {
            apple: Apple::new(&snake),
            snake,
            score: 0,
            update_lock: false,
            respawn: false,
            moves: 0,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, mut gl: GlGraphics) {
        gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BACKGROUND_COLOR, gl);
        });

        self.apple.render(&mut gl, args);
        self.snake.render(&mut gl, args);
    }

    pub fn update(&mut self) {
        if self.snake.dead {
            self.snake = Snake::new();
        }

        self.snake.update();

        if self.snake.score(&self.apple) {
            self.score = self.snake.score;
            self.apple = Apple::new(&self.snake);
        }

        // Clear update lock to allow movement
        self.update_lock = false;

        self.moves += 1;
    }

    pub fn pressed(&mut self, btn: &Button) {
        // Make sure the player can't move more than once per tick
        if !self.update_lock {
            let last_direction = self.snake.dir.clone();

            self.snake.dir = match btn {
                &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
                &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
                &Button::Keyboard(Key::Left) if last_direction != Direction::Right => {
                    Direction::Left
                }
                &Button::Keyboard(Key::Right) if last_direction != Direction::Left => {
                    Direction::Right
                }
                _ => last_direction,
            };
        } else {
            // Warn if the player moves before next tick
            println!("Ignored key press, waiting for update...");
        }

        // Tell the game not to listen for key presses
        self.update_lock = true;
    }
}
