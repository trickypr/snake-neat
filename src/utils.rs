use std::cmp::Ordering;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

////////////////////////////////
// Helper functions
pub fn in_rect(x1: i32, y1: i32, x2: i32, y2: i32, x: i32, y: i32) -> bool {
    x >= x1 && x <= x2 && y >= y1 && y <= y2
}

////////////////////////////////
// Direction enum
#[derive(Clone, PartialEq, FromPrimitive, Debug)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn value(&self) -> f64 {
        match *self {
            Direction::Up => 0f64,
            Direction::Down => 0.33f64,
            Direction::Left => 0.66f64,
            Direction::Right => 1f64,
        }
    }

    pub fn from_vec(input: &Vec<f64>) -> Direction {
        let index_of_max = input
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();

        Direction::from_i32(index_of_max as i32).unwrap()
    }
}

////////////////////////////////
// Area area
pub const ARENA_WIDTH: u32 = 20;
pub const ARENA_HEIGHT: u32 = 20;

////////////////////////////////
// Colors

// Colors for background
pub const BACKGROUND_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0]; // Dark grey

// Colors for player
pub const SNAKE_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0]; // Light grey
pub const DEAD_COLOR: [f32; 4] = [0.7, 0.1, 0.1, 1.0]; // Red

// Colors for apple
pub const APPLE_COLOR: [f32; 4] = [0.9, 0.5, 0.0, 1.0]; // Orange

////////////////////////////////
// Tests for file
#[cfg(test)]
mod Tests {
    use crate::utils::*;

    #[test]
    fn direction_from_vec() {
        let test_vec = vec![0f64, 0.8f64, 0.9f64, 0.2f64];

        assert_eq!(Direction::from_vec(&test_vec), Direction::Left);
    }
}
