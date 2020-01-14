use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use super::gamestate::GameState;
use super::states::State;
use super::gamedata::GameData;

pub struct EndGame {
    gl: GlGraphics,
    size: i8,
    go_to_next_state: bool,
    data: GameData
}

impl EndGame{
    pub fn new(opengl_version: OpenGL, board_size: i8, data: GameData) -> EndGame {
        EndGame {
            gl: GlGraphics::new(opengl_version),
            size: board_size,
            go_to_next_state: false,
            data: data
        }
    }
}

impl GameState for EndGame{
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){
            use graphics::*;
           
            const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let u_score = self.data.score.to_string();

            self.gl.draw(args.viewport(), |c, gl| {
                let transform_game_over = c.transform.trans(250.0, 350.0);
                let transform_score =  c.transform.trans(250.0, 400.0);
                clear(GRAY, gl);

                text::Text::new_color(RED, 32).draw(
                    "GAME OVER",
                    glyphs,
                    &c.draw_state,
                    transform_game_over,
                    gl
                ).unwrap();

                text::Text::new_color(RED, 32).draw(
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