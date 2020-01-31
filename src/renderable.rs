use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;

pub trait Renderable {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics);
}