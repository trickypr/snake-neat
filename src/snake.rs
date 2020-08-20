extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use std::collections::LinkedList;
use std::iter::FromIterator;

use crate::apple::*;
use crate::utils::*;

pub struct Snake {
    pub dir: Direction,
    pub score: u32,
    pub dead: bool,
    pub body: LinkedList<(i32, i32)>,
    last_removed: (i32, i32),
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right,
            dead: false,
            score: 0,
            last_removed: (0, 0),
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter().for_each(|square| {
                graphics::rectangle(
                    if self.dead { DEAD_COLOR } else { SNAKE_COLOR },
                    square,
                    transform,
                    gl,
                );
            });
        });
    }

    pub fn score(&mut self, apple: &Apple) -> bool {
        let head = self.body.front().expect("Snake has no body!");

        if head.0 == apple.x && head.1 == apple.y {
            self.score += 1;

            self.body.push_back(self.last_removed);

            return true;
        }

        false
    }

    fn head_intersect(&self) -> bool {
        let head = self.body.front().expect("Snake has no body!");
        let mut body = self.body.clone();

        let mut intersection = false;

        body.pop_front();
        body.iter().for_each(|segment| {
            if !intersection && segment.0 == head.0 && segment.1 == head.1 {
                intersection = true;
            }
        });

        intersection
    }

    pub fn body_overlap(&self, x: i32, y: i32) -> bool {
        let mut intersection = false;

        self.body.iter().for_each(|segment| {
            if !intersection && segment.0 == x && segment.1 == y {
                intersection = true;
            }
        });

        intersection
    }

    fn gen_head(&self) -> (i32, i32) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        // Set its position
        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        new_head
    }

    pub fn update(&mut self) {
        // Don't move snake if dead
        if !self.dead {
            // Create a new head
            let new_head = self.gen_head();

            // Check if snake is dead
            // in_rect checks if it outside the world bounds
            // head_intersect checks if the head is interscting the body
            if !in_rect(
                0,
                0,
                (ARENA_WIDTH - 1) as i32,
                (ARENA_HEIGHT - 1) as i32,
                new_head.0,
                new_head.1,
            ) || self.head_intersect()
            {
                self.dead = true;
            } else {
                // Move segments
                self.body.push_front(new_head);
                self.last_removed = self.body.pop_back().unwrap();
            }
        }
    }
}
