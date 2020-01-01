use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use super::gamestate::GameState;

pub struct StartGame {
    gl: GlGraphics,
    size: i8,
}

impl StartGame{
    pub fn new(opengl_version: OpenGL, board_size: i8) -> StartGame {
        StartGame {
            gl: GlGraphics::new(opengl_version),
            size: board_size,
        }
    }
}

impl GameState for StartGame{
    fn render(&mut self, args: &RenderArgs){
            use graphics::*;

            const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            self.gl.draw(args.viewport(), |c, gl| {
                clear(GRAY, gl);
            });
    }

    fn update(&mut self, args: &UpdateArgs){
            
    }

    fn key_press(&mut self, args: &Button){
            match *args {
                Keyboard(Key::Space) => {/*TODO: Start Game*/},
                _ => {/* Do nothing */}
            }
    }
}