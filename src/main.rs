////////////////////////////////
// Graphics crates
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

////////////////////////////////
// AI crates
extern crate rustneat;

////////////////////////////////
// Graphics use
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

////////////////////////////////
// AI Use
use rustneat::Organism;
use rustneat::Population;

////////////////////////////////
// Import everything
mod apple;
mod enviroment;
mod game;
mod snake;
mod telemetry_helper;
mod utils;

fn main() {
    // #[cfg(feature = "telemetry")]
    telemetry_helper::enable_telemetry("", true);

    // #[cfg(feature = "telemetry")]
    std::thread::sleep(std::time::Duration::from_millis(2000));

    let mut population = Population::create_population(500);
    let mut environment = enviroment::GameEnvironment::new();
    let mut champion: Option<Organism> = None;

    while champion.is_none() {
        population.evolve();
        population.evaluate_in(&mut environment);

        let mut best_fitness = 0.0;

        for organism in &population.get_organisms() {
            if organism.fitness > best_fitness {
                best_fitness = organism.fitness;
            }

            if organism.fitness > 15.9f64 {
                champion = Some(organism.clone());
            }
        }

        println!("Best gen fitness: {}", best_fitness);
    }

    println!("{:?}", champion.unwrap().genome);

    // let opengl = OpenGL::V3_2;

    // let mut window: Window = WindowSettings::new(
    //     "Snek Ai",
    //     [utils::ARENA_WIDTH * 20, utils::ARENA_HEIGHT * 20],
    // )
    // .graphics_api(opengl)
    // .exit_on_esc(true)
    // .build()
    // .unwrap();

    // let mut game = game::Game::new(GlGraphics::new(opengl), snake::Snake::new());

    // let mut event_settings = EventSettings::new();
    // event_settings.ups = 6;

    // let mut events = Events::new(event_settings);
    // while let Some(e) = events.next(&mut window) {
    //     if let Some(r) = e.render_args() {
    //         game.render(&r);
    //     }

    //     if let Some(_u) = e.update_args() {
    //         game.update();
    //     }

    //     if let Some(k) = e.button_args() {
    //         if k.state == ButtonState::Press {
    //             game.pressed(&k.button);
    //         }
    //     }
    // }
}
