use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use super::snake::Snake;
use super::points::Points;
use super::gamestate::GameState;
use super::renderable::Renderable;
use super::userscore::{UserScore, HighScores};
use super::states::State;
use super::gamedata::GameData;
use super::textwriter::TextWriter;
use super::colors;
use super::config;
use std::fs::File;
use std::io::prelude::*;
use super::scorecontroller::ScoreController;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Game {
    gl: GlGraphics,
    writer: TextWriter,
    snake: Snake,
    size: i8,
    points: Points,
    score: i32,
    level: f64,
    elapsed: f64,
    score_controller: Rc<RefCell<ScoreController>>,
    data: GameData,
    is_Paused: bool,
}

impl Game{
    pub fn new(opengl_version: OpenGL, board_size: i8, data: GameData, score_controller: Rc<RefCell<ScoreController>>) -> Game {
        Game {
            gl: GlGraphics::new(opengl_version),
            snake: Snake::new(),
            writer: TextWriter::new(),
            size: board_size,
            points: Points::new(board_size),
            score: 0,
            elapsed: 0.0,
            level: 0.3,
            score_controller: score_controller,
            data: data,
            is_Paused: false
        }
    }

    fn process_point_scored(&mut self){
        let (x, y) = self.snake.tail[0];

        if self.points.collision(x, y){
            self.points.next();
            self.snake.add_tail_element();
            self.score += 10;
            self.level *= 0.95;
        }
    }

    fn process_game_over(&mut self) -> bool {
        if self.snake.self_collision(){
            (*self.score_controller.borrow_mut()).add_new_score(&self.data.username, self.score);
            return true;
        }
        return false;
    }

    fn should_process_update(&mut self, delta: f64) -> bool {
        if self.is_Paused {
            return false;
        }

        self.elapsed += delta;
        if self.elapsed < self.level {
            return false;
        }
        self.elapsed = 0.0;

        return true;
    }

    fn render_score(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){

        let score_string = self.score.to_string();
        let high_score_string = (*self.score_controller.borrow_mut()).get_high_score();
        let score_combined = &("Score: ".to_owned() + &score_string + " High score: " + &high_score_string);

        self.writer.render_text(args, &mut self.gl, glyphs, colors::BLACK, 24, 10.0, 25.0, score_combined);
    }

    fn render_pause(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){
        if !self.is_Paused {
            return;
        }
        self.writer.render_text(args, &mut self.gl, glyphs, colors::RED, 24, 690.0, 25.0, "PAUSED");
    }
}

impl GameState for Game{
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){
            use graphics::*;

            self.gl.draw(args.viewport(), |c, gl| {
                clear(colors::GRAY, gl);
            });

            self.snake.render(args, &mut self.gl);
            self.points.render(args, &mut self.gl);
            self.render_score(args, glyphs);
            self.render_pause(args, glyphs);
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {

            if !self.should_process_update(args.dt){
                return State::None;
            }

            let start = SystemTime::now();
            let size = self.size;

            self.snake.update_position(|x| { if x == -1 {size - 1} else { x % size}});
            if self.process_game_over() {
                let mut gamedata = GameData::new();
                gamedata.username = self.data.username.clone();
                gamedata.score = self.score;
                return State::End(gamedata);
            }
            self.process_point_scored();

            return State::None;
    }

    fn key_press(&mut self, args: &Button){
            match *args {
                Keyboard(Key::W) | Keyboard(Key::Up) => self.snake.go_up(),
                Keyboard(Key::S) | Keyboard(Key::Down) => self.snake.go_down(),
                Keyboard(Key::A) | Keyboard(Key::Left) => self.snake.go_left(),
                Keyboard(Key::D) | Keyboard(Key::Right) => self.snake.go_right(),
                Keyboard(Key::P) => self.is_Paused = !self.is_Paused,
                _ => {/* Do nothing */}
            }
    }
}