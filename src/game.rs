use piston::input::{UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::Context;
use super::snake::Snake;
use super::points::Points;
use super::gamestate::GameState;
use super::renderable::Renderable;
use super::states::State;
use super::gamedata::GameData;
use super::textwriter::TextWriter;
use super::colors;
use super::scorecontroller::ScoreController;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Game {
    writer: TextWriter,
    snake: Snake,
    size: i16,
    board_size: i16,
    points: Points,
    score: i32,
    level: f64,
    elapsed: f64,
    score_controller: Rc<RefCell<ScoreController>>,
    data: GameData,
    is_Paused: bool,
}

impl Game{
    pub fn new(board_size: i16, step_size: i16, data: GameData, score_controller: Rc<RefCell<ScoreController>>) -> Game {
        
        if (board_size % step_size) != 0 {
            panic!("Incorrect size or step size {0}", (board_size % step_size));
        }

        let size = (board_size / step_size);

        Game {
            snake: Snake::new(),
            writer: TextWriter::new(),
            size: size,
            board_size: board_size,
            points: Points::new(size),
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

    fn render_score(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){

        let score_string = self.score.to_string();
        let high_score_string = (*self.score_controller.borrow_mut()).get_high_score();
        let score_combined = &("Score: ".to_owned() + &score_string + " High score: " + &high_score_string);

        self.writer.render_text(ctx, &mut gl, glyphs, colors::BLACK, 24, 10.0, 25.0, score_combined);
    }

    fn render_pause(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
        if !self.is_Paused {
            return;
        }
        self.writer.render_text(ctx, &mut gl, glyphs, colors::RED, 24, 690.0, 25.0, "PAUSED");
    }

    fn render_board_background(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.board_size as f64);
        rectangle(colors::DARK_GRAY, square, ctx.transform, gl);
    }
}

impl GameState for Game{
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
            self.render_board_background(&ctx, &mut gl);
            self.snake.render(&ctx, &mut gl);
            self.points.render(&ctx, &mut gl);
            self.render_score(&ctx, &mut gl, glyphs);
            self.render_pause(&ctx, &mut gl, glyphs);       
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {

            if !self.should_process_update(args.dt){
                return State::None;
            }

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