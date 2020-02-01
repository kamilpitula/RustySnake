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
use scorecontroller::ScoreController;
use std::rc::Rc;
use std::cell::RefCell;

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
mod colors;
mod config;
mod renderable;
mod textwriter;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Rusty Snake", [800, 800])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut current_state: Box<dyn GameState> = Box::new(startgame::StartGame::new(opengl, 32));
    let score_controller = Rc::new(RefCell::new(ScoreController::new()));

    let mut events = get_events_loop();
    let mut glyph_cache = get_font();

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            current_state.render(&args, &mut glyph_cache);
        }

        if let Some(args) = e.update_args(){
            let stateFinished = current_state.update(&args);
            
            current_state = 
            match stateFinished {
                State::Start(data) =>{Box::new(startgame::StartGame::new(opengl, 32))},
                State::Game(data) => {Box::new(game::Game::new(opengl, 32, data, Rc::clone(&score_controller)))},
                State::End(data) => {Box::new(endgame::EndGame::new(opengl, 32, data, Rc::clone(&score_controller)))},
                State::None => {current_state},
            }
        }

        if let Some(args) = e.press_args(){
            current_state.key_press(&args);
        }
    }
}

fn get_font() -> GlyphCache<'static> {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
                        .for_folder("assets")
                        .unwrap();

    let font_path = assets.join("AllertaStencil-Regular.ttf");

    GlyphCache::new(&font_path, (), TextureSettings::new()).unwrap()
}

fn get_events_loop() -> Events {

    let mut settings = EventSettings::new();
    settings.ups = config::UPS;
    settings.max_fps = config::MAX_FPS;

    Events::new(settings)
}
