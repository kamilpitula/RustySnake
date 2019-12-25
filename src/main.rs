extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, PressEvent, UpdateEvent};
use piston::window::WindowSettings;

mod game;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(opengl);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            game.render(&args);
        }

        if let Some(args) = e.update_args(){
            game.update(&args);
        }

        if let Some(args) = e.press_args(){
            game.key_press(&args);
        }
    }
}
