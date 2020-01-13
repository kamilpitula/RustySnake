use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use super::gamestate::GameState;
use super::states::State;
use super::gamedata::GameData;


pub struct StartGame {
    gl: GlGraphics,
    size: i8,
    go_to_next_state: bool,
    username: String
}

impl StartGame{
    pub fn new(opengl_version: OpenGL, board_size: i8) -> StartGame {
        StartGame {
            gl: GlGraphics::new(opengl_version),
            size: board_size,
            go_to_next_state: false,
            username: String::new()
        }
    }
}

impl GameState for StartGame{
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){
            use graphics::*;

            const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let u_name = &self.username;

            self.gl.draw(args.viewport(), |c, gl| {
                let transform_press_space = c.transform.trans(250.0, 400.0);
                let transform_username = c.transform.trans(250.0, 350.0);
                clear(GRAY, gl);

                text::Text::new_color(RED, 32).draw(
                    "Press enter to start",
                    glyphs,
                    &c.draw_state,
                    transform_press_space,
                    gl
                ).unwrap();

                text::Text::new_color(RED, 32).draw(
                    &("Username: ".to_owned() + &u_name),
                    glyphs,
                    &c.draw_state,
                    transform_username,
                    gl
                ).unwrap();

            });
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData>{
            if self.go_to_next_state {
                let mut data = GameData::new();
                data.username = self.username.clone();
                if data.username.len() < 1{
                    return State::None;
                }
                return State::Game(data);
            }
            return State::None;
    }

    fn key_press(&mut self, args: &Button){
            match *args {
                Keyboard(Key::Return) => { self.go_to_next_state = self.username.len() > 0 },
                Keyboard(Key::A) => { self.username.push('A') },
                Keyboard(Key::B) => { self.username.push('B') },
                Keyboard(Key::C) => { self.username.push('C') },
                Keyboard(Key::D) => { self.username.push('D') },
                Keyboard(Key::E) => { self.username.push('E') },
                Keyboard(Key::F) => { self.username.push('F') },
                Keyboard(Key::G) => { self.username.push('G') },
                Keyboard(Key::H) => { self.username.push('H') },
                Keyboard(Key::I) => { self.username.push('I') },
                Keyboard(Key::J) => { self.username.push('J') },
                Keyboard(Key::K) => { self.username.push('K') },
                Keyboard(Key::L) => { self.username.push('L') },
                Keyboard(Key::M) => { self.username.push('M') },
                Keyboard(Key::N) => { self.username.push('N') },
                Keyboard(Key::O) => { self.username.push('O') },
                Keyboard(Key::P) => { self.username.push('P') },
                Keyboard(Key::Q) => { self.username.push('Q') },
                Keyboard(Key::R) => { self.username.push('R') },
                Keyboard(Key::S) => { self.username.push('S') },
                Keyboard(Key::T) => { self.username.push('T') },
                Keyboard(Key::U) => { self.username.push('U') },
                Keyboard(Key::V) => { self.username.push('V') },
                Keyboard(Key::W) => { self.username.push('W') },
                Keyboard(Key::X) => { self.username.push('X') },
                Keyboard(Key::Y) => { self.username.push('Y') },
                Keyboard(Key::Z) => { self.username.push('Z') },
                Keyboard(Key::Backspace) => { self.username.pop(); },
                _ => {/* Do nothing */}
            }
    }
}