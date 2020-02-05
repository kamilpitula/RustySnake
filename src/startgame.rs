use piston::input::{UpdateArgs, Button};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use graphics::Context;
use super::gamestate::GameState;
use super::states::State;
use super::gamedata::GameData;
use super::textwriter::TextWriter;
use super::colors;


pub struct StartGame {
    writer: TextWriter,
    size: i8,
    go_to_next_state: bool,
    username: String
}

impl StartGame{
    pub fn new(board_size: i8) -> StartGame {
        StartGame {
            writer: TextWriter::new(),
            size: board_size,
            go_to_next_state: false,
            username: String::new()
        }
    }
}

impl GameState for StartGame{
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
            
        let u_name = &self.username;

        let name_to_write =  &("Player name: ".to_owned() + &u_name);

        self.writer.render_text(&ctx, &mut gl, glyphs, colors::RED, 32, 250.0, 400.0, "Press enter to start");
        self.writer.render_text(&ctx, &mut gl, glyphs, colors::RED, 32, 250.0, 350.0, name_to_write);
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