use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;

pub struct TextWriter{
}

impl TextWriter {
    pub fn new() -> TextWriter {
        TextWriter {
        }
    }

    pub fn write(
        &mut self,
        args: &RenderArgs,
        gl: &mut GlGraphics,
        glyphs: &mut GlyphCache,
        color: [f32; 4],
        size: u32,
        pos_x: f64,
        pos_y: f64,
        to_Write: &str) {

        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {

            let text_trans = c
                    .transform
                    .trans(pos_x, pos_y);

                text::Text::new_color(color, size).draw(
                    to_Write,
                    glyphs,
                    &c.draw_state,
                    text_trans,
                    gl
                ).unwrap();
        });

    }
}