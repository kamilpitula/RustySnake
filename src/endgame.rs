use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;

use super::scorecontroller::ScoreController;
use super::gamestate::GameState;
use super::states::State;
use super::colors;
use super::gamedata::GameData;
use super::textwriter::TextWriter;
use std::rc::Rc;
use std::cell::RefCell;

pub struct EndGame {
    gl: GlGraphics,
    writer: TextWriter,
    size: i8,
    go_to_next_state: bool,
    data: GameData,
    top_ten: Vec<String>
}

impl EndGame{
    pub fn new(opengl_version: OpenGL, board_size: i8, data: GameData, score_controller: Rc<RefCell<ScoreController>>) -> EndGame {
        EndGame {
            gl: GlGraphics::new(opengl_version),
            writer: TextWriter::new(),
            size: board_size,
            go_to_next_state: false,
            data: data,
            top_ten: (*score_controller.borrow_mut()).get_top_scores(10)
        }
    }

    fn render_your_score(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){

        let u_score = self.data.score.to_string();
        let your_score =  &("Your score: ".to_owned() + &u_score);
        self.writer.render_text(args, &mut self.gl, glyphs, colors::RED, 42, 250.0, 70.0, "GAME OVER");

        self.writer.render_text(args, &mut self.gl, glyphs, colors::RED, 32, 250.0, 120.0, your_score);
    }

    fn render_top_scores(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){

        let scores = &self.top_ten;
        for (index, score) in scores.iter().enumerate() {
            let score = &((index + 1).to_string() + ". " + &score);
            let pos_y = 260 + (30 * index);

            self.writer.render_text(args, &mut self.gl, glyphs, colors::BLACK, 24, 250.0, pos_y as f64, score);
        }
    }
}

impl GameState for EndGame{
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){
            use graphics::*;

            self.gl.draw(args.viewport(), |c, gl| {
                clear(colors::GRAY, gl);
            });
            
            self.render_your_score(args, glyphs);
            self.render_top_scores(args, glyphs);
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData>{
            if self.go_to_next_state {
                return State::Start(GameData::new())
            }
            return State::None;
    }

    fn key_press(&mut self, args: &Button){
            match *args {
                Keyboard(Key::Return) => { self.go_to_next_state = true; },
                _ => {/* Do nothing */}
            }
    }
}
