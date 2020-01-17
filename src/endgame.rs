use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;

use super::scorecontroller::ScoreController;
use super::gamestate::GameState;
use super::states::State;
use super::colors;
use super::gamedata::GameData;
use std::rc::Rc;
use std::cell::RefCell;

pub struct EndGame {
    gl: GlGraphics,
    size: i8,
    go_to_next_state: bool,
    data: GameData,
    top_ten: Vec<String>
}

impl EndGame{
    pub fn new(opengl_version: OpenGL, board_size: i8, data: GameData, score_controller: Rc<RefCell<ScoreController>>) -> EndGame {
        EndGame {
            gl: GlGraphics::new(opengl_version),
            size: board_size,
            go_to_next_state: false,
            data: data,
            top_ten: (*score_controller.borrow_mut()).get_top_scores(10)
        }
    }
}

impl GameState for EndGame{
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){
            use graphics::*;

            let u_score = self.data.score.to_string();
            let scores = &self.top_ten;

            self.gl.draw(args.viewport(), |c, gl| {
                let transform_game_over = c.transform.trans(250.0, 70.0);
                let transform_score =  c.transform.trans(250.0, 120.0);
                clear(colors::GRAY, gl);
                let scores_position = 260;

                for (index, score) in scores.iter().enumerate() {
                    let transform_high = c.transform.trans(250.0, (scores_position + (30 * index)) as f64);

                    text::Text::new_color(colors::BLACK, 24).draw(
                        &((index + 1).to_string() + ". " + &score),
                        glyphs,
                        &c.draw_state,
                        transform_high,
                        gl
                    ).unwrap();
                }

                text::Text::new_color(colors::RED, 42).draw(
                    "GAME OVER",
                    glyphs,
                    &c.draw_state,
                    transform_game_over,
                    gl
                ).unwrap();

                text::Text::new_color(colors::RED, 32).draw(
                    &("Your score: ".to_owned() + &u_score),
                    glyphs,
                    &c.draw_state,
                    transform_score,
                    gl
                ).unwrap();

            });
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
