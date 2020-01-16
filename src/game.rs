use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use super::snake::Snake;
use super::points::Points;
use super::gamestate::GameState;
use super::userscore::{UserScore, HighScores};
use super::states::State;
use super::gamedata::GameData;
use super::colors;
use std::fs::File;
use std::io::prelude::*;
use super::scorecontroller::ScoreController;


const STEP: f64 = 25.0;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    size: i8,
    points: Points,
    score: i32,
    level: f64,
    score_controller: ScoreController,
    data: GameData
}

impl Game{
    pub fn new(opengl_version: OpenGL, board_size: i8, data: GameData) -> Game {
        Game {
            gl: GlGraphics::new(opengl_version),
            snake: Snake::new(),
            size: board_size,
            points: Points::new(board_size),
            score: 0,
            level: 200.0,
            score_controller: ScoreController::new(),
            data: data
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
            self.score_controller.add_new_score(&self.data.username, self.score);
            return true;
        }
        return false;
    }

    fn fill_unused_frame_time(&mut self, frame_start_time: &SystemTime){
            let difference = frame_start_time.duration_since(*frame_start_time)
                                                .expect("Calculating remaining time failed");

            let sum_with_frameduration = difference.checked_add(Duration::from_millis(self.level as u64));

            match sum_with_frameduration {
                Some(n) => sleep(n),
                None => panic!("Not an option")
            }
    }
}

impl GameState for Game{
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache){
            use graphics::*;

            let square = rectangle::square(0.0, 0.0, STEP);
            let tail = &self.snake.tail;
            let point_x = self.points.position_x;
            let point_y = self.points.position_y;
            let score_string = self.score.to_string();
            let high_score_string = self.score_controller.get_high_score();

            self.gl.draw(args.viewport(), |c, gl| {
                clear(colors::GRAY, gl);

                let point_trans = c
                    .transform
                    .trans(point_x as f64 * STEP, point_y as f64 * STEP);
                
                let text_trans = c
                    .transform
                    .trans(10.0, 25.0);
                
                rectangle(colors::BLUE, square, point_trans, gl);

                for (x, y) in tail {
                    let transform = c
                    .transform
                    .trans(*x as f64 * STEP, *y as f64 * STEP);

                    rectangle(colors::RED, square, transform, gl);
                }

                text::Text::new_color(colors::BLACK, 24).draw(
                    &("Score: ".to_owned() + &score_string + " High score: " + &high_score_string),
                    glyphs,
                    &c.draw_state,
                    text_trans,
                    gl
                ).unwrap();
            });
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
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

            self.fill_unused_frame_time(&start);

            return State::None;
    }

    fn key_press(&mut self, args: &Button){
            match *args {
                Keyboard(Key::W) => self.snake.go_up(),
                Keyboard(Key::S) => self.snake.go_down(),
                Keyboard(Key::A) => self.snake.go_left(),
                Keyboard(Key::D) => self.snake.go_right(),
                _ => {/* Do nothing */}
            }
    }
}