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
use states::State;

mod gamestate;
mod startgame;
mod game;
mod endgame;
mod snake;
mod points;
mod userscore;
mod states;
mod gamedata;
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
    
    let mut current_state: Box<GameState> = Box::new(startgame::StartGame::new(opengl, 32));

    let mut settings = EventSettings::new();
    settings.ups = 60;
    settings.max_fps = 60;
    let mut events = Events::new(settings);

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            current_state.render(&args, &mut glyph_cache);
        }

        if let Some(args) = e.update_args(){
            let stateFinished = current_state.update(&args);
            current_state = 
            match stateFinished {
                State::Start(data) =>{Box::new(startgame::StartGame::new(opengl, 32))},
                State::Game(data) => {Box::new(game::Game::new(opengl, 32))},
                State::End(data) => {Box::new(endgame::EndGame::new(opengl, 32))},
                State::None => {current_state},
            }
        }

        if let Some(args) = e.press_args(){
            current_state.key_press(&args);
        }
    }
}
