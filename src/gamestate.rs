use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::{OpenGL, GlyphCache};
use piston_window::Glyphs;

pub trait GameState {
        fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache);
        fn update(&mut self, args: &UpdateArgs) -> bool;
        fn key_press(&mut self, args: &Button);
}