extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate itertools;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, PressEvent, UpdateEvent};
use piston::window::WindowSettings;
use gamestate::GameState;
use std::rc::Rc;

mod gamestate;
mod startgame;
mod game;
mod snake;
mod points;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Rusty Snake", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    
    let mut start_view = Box::new(startgame::StartGame::new(opengl, 32));
    let mut game_view = Box::new(game::Game::new(opengl, 32));

    let mut states: Vec<Box<GameState>> = vec![start_view, game_view];

    let mut current_state = 0;


    let mut settings = EventSettings::new();
    settings.ups = 60;
    settings.max_fps = 60;
    let mut events = Events::new(settings);

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            states[current_state].render(&args);
        }

        if let Some(args) = e.update_args(){
            states[current_state].update(&args);
        }

        if let Some(args) = e.press_args(){
            states[current_state].key_press(&args);
        }
    }
}
