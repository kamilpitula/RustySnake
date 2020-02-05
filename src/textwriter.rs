use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::Context;

pub struct TextWriter{
}

impl TextWriter {
    pub fn new() -> TextWriter {
        TextWriter {
        }
    }

    pub fn render_text(
        &mut self,
        ctx: &Context,
        gl: &mut GlGraphics,
        glyphs: &mut GlyphCache,
        color: [f32; 4],
        size: u32,
        pos_x: f64,
        pos_y: f64,
        to_Write: &str) {
        use graphics::*;

        let text_trans = ctx
                .transform
                .trans(pos_x, pos_y);

            text::Text::new_color(color, size).draw(
                to_Write,
                glyphs,
                &ctx.draw_state,
                text_trans,
                gl
            ).unwrap();
    }
}