////////////////////////////////
// AI crates
extern crate rustneat;

////////////////////////////////
// AI Use
use rustneat::Environment;
use rustneat::Organism;

////////////////////////////////
// Game imports
use crate::game::*;
use crate::snake::*;
use crate::utils::*;

pub struct GameEnvironment {}

impl GameEnvironment {
    pub fn new() -> GameEnvironment {
        GameEnvironment {}
    }

    fn generate_map(&self, game: &Game) -> Vec<f64> {
        let mut map = [[0f64; ARENA_WIDTH as usize]; ARENA_HEIGHT as usize];

        // Add snake head
        let snake_head = game.snake.body.front().expect("Snake has no body!");
        map[snake_head.0 as usize][snake_head.1 as usize] = 0.75f64;

        // Add snake body
        let mut snake_body = game.snake.body.clone();
        snake_body.pop_front();
        snake_body.iter().for_each(|segment| {
            map[segment.0 as usize][segment.1 as usize] = 0.5f64;
        });

        // Add apple
        map[game.apple.x as usize][game.apple.y as usize] = 1f64;

        // Convert to a flat vector
        let mut input: Vec<f64> = Vec::new();
        for x in map.iter() {
            for y in x.iter() {
                input.push(y.clone());
            }
        }

        input
    }
}

impl Environment for GameEnvironment {
    fn test(&self, organism: &mut Organism) -> f64 {
        let mut output = vec![0f64; 4];

        let mut game = Game::new(Snake::new());

        while !game.snake.dead {
            // Construct the map
            let mut input = self.generate_map(&game);

            // Provide the AI with the current direction
            input.push(game.snake.dir.value());

            organism.activate(input, &mut output);

            game.snake.dir = Direction::from_vec(&output);
            game.update();
        }

        (game.score - (game.moves / 1000)) as f64
    }
}
