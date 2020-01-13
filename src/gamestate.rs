use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::GlyphCache;
use super::states::State;
use super::gamedata::GameData;

pub trait GameState {
        fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache);
        fn update(&mut self, args: &UpdateArgs) -> State<GameData>;
        fn key_press(&mut self, args: &Button);
}