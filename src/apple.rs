use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::Rng;

use crate::snake::*;
use crate::utils::*;

#[derive(Debug)]
pub struct Apple {
    pub x: i32,
    pub y: i32,
}

impl Apple {
    pub fn new(snake: &Snake) -> Apple {
        let mut rng = rand::thread_rng();

        let mut x = rng.gen_range(0, ARENA_WIDTH) as i32;
        let mut y = rng.gen_range(0, ARENA_HEIGHT) as i32;

        while snake.body_overlap(x, y) {
            x = rng.gen_range(0, ARENA_WIDTH) as i32;
            y = rng.gen_range(0, ARENA_HEIGHT) as i32;
        }

        Apple { x, y }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square =
            graphics::rectangle::square((self.x * 20 + 5) as f64, (self.y * 20 + 5) as f64, 10_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(APPLE_COLOR, square, transform, gl);
        });
    }
}
