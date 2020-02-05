use opengl_graphics::GlGraphics;
use graphics::Context;

pub trait Renderable {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics);
}