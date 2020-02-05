extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate itertools;
extern crate find_folder;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, GlGraphics, GlyphCache};
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
    
    let mut current_state: Box<dyn GameState> = Box::new(startgame::StartGame::new(32));
    let score_controller = Rc::new(RefCell::new(ScoreController::new()));

    let mut events = get_events_loop();
    let mut glyph_cache = get_font();

    let mut gl = GlGraphics::new(opengl);
    let (ax, ay) = config::TARGET_ASPECT;

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){

            gl.draw(args.viewport(), |c, mut gl| {
                clear(colors::GRAY, gl);

                let (board_size_calculated, left, bottom) = calculate_viewport(&c);

                let c = c.scale(
                    board_size_calculated / config::BOARD_SIZE as f64,
                    board_size_calculated / config::BOARD_SIZE as f64);
                let c = c.trans(left, bottom);

                current_state.render(&c, &mut gl, &mut glyph_cache);
            });
        }

        if let Some(args) = e.update_args(){
            let stateFinished = current_state.update(&args);
            
            current_state = 
            match stateFinished {
                State::Start(data) =>{Box::new(startgame::StartGame::new(32))},
                State::Game(data) => {Box::new(game::Game::new(config::BOARD_SIZE, config::STEP, data, Rc::clone(&score_controller)))},
                State::End(data) => {Box::new(endgame::EndGame::new(32, data, Rc::clone(&score_controller)))},
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

fn calculate_viewport(ctx: &Context) -> (f64, f64, f64) {
    let size = ctx.get_view_size();
    let size_x = size[0];
    let size_y = size[1];
    // let width = size_x.min((size_y * ax as f64) / ay as f64);
    // let height = size_y.min((size_x * ay as f64) / ax as f64);
    let width = size_x.min(config::BOARD_SIZE as f64);
    let height = size_y.min(config::BOARD_SIZE as f64);
    let a = width.min(height);
    let left = (size_x - width) / 2.0;
    let bottom = (size_y - height) / 2.0;
    
    (a, left, bottom)
}
