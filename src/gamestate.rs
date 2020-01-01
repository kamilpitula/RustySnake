use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::OpenGL;

pub trait GameState {
        fn render(&mut self, args: &RenderArgs);
        fn update(&mut self, args: &UpdateArgs);
        fn key_press(&mut self, args: &Button);
}