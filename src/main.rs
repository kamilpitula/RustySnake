extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate itertools;
extern crate find_folder;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, GlyphCache};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, PressEvent, UpdateEvent};
use piston::window::*;
use piston_window::*;
use gamestate::GameState;

mod gamestate;
mod startgame;
mod game;
mod endgame;
mod snake;
mod points;
mod userscore;
mod scorecontroller;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Rusty Snake", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
                        .for_folder("assets")
                        .unwrap();

    let font_path = assets.join("AllertaStencil-Regular.ttf");

    let mut glyph_cache = GlyphCache::new(&font_path, (), TextureSettings::new()).unwrap();
    
    let start_view = Box::new(startgame::StartGame::new(opengl, 32));
    let game_view = Box::new(game::Game::new(opengl, 32));
    let end_view = Box::new(endgame::EndGame::new(opengl, 32));

    let mut states: Vec<Box<GameState>> = vec![start_view, game_view, end_view];

    let mut current_state = 0;


    let mut settings = EventSettings::new();
    settings.ups = 60;
    settings.max_fps = 60;
    let mut events = Events::new(settings);

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            states[current_state].render(&args, &mut glyph_cache);
        }

        if let Some(args) = e.update_args(){
            let state_finished = states[current_state].update(&args);
            if state_finished{
                current_state += 1;
            }
        }

        if let Some(args) = e.press_args(){
            states[current_state].key_press(&args);
        }
    }
}
