use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use opengl_graphics::{GlGraphics, OpenGL};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use super::snake::Snake;

const STEP: f64 = 25.0;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    size: i8
}

impl Game{
    pub fn new(opengl_version: OpenGL, board_size: i8) -> Game {
        Game {
            gl: GlGraphics::new(opengl_version),
            snake: Snake::new(),
            size: board_size
        }
    }

    pub fn render(&mut self, args: &RenderArgs){
            use graphics::*;

            const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, STEP);
            let position_x = self.snake.position_x;
            let position_y = self.snake.position_y;

            self.gl.draw(args.viewport(), |c, gl| {
                clear(GRAY, gl);

                let transform = c
                    .transform
                    .trans(position_x as f64 * STEP, position_y as f64 * STEP);

                rectangle(RED, square, transform, gl);
            });
            println!("render");
    }

    pub fn update(&mut self, args: &UpdateArgs){
            let start = SystemTime::now();
            let size = self.size;
            self.snake.update_position(|x| { if x == -1 {size - 1} else{ x % size}});
            println!("update");
            self.fill_unused_frame_time(&start);
    }

    pub fn key_press(&mut self, args: &Button){
            match *args {
                Keyboard(Key::W) => self.snake.go_up(),
                Keyboard(Key::S) => self.snake.go_down(),
                Keyboard(Key::A) => self.snake.go_left(),
                Keyboard(Key::D) => self.snake.go_right(),
                _ => {/* Do nothing */}
            }
    }

    fn fill_unused_frame_time(&mut self, frame_start_time: &SystemTime){
            let difference = frame_start_time.duration_since(*frame_start_time)
                                                .expect("Calculating remaining time failed");
        
            let sum_with_frameduration = difference.checked_add(Duration::from_millis(20));

            match sum_with_frameduration {
                Some(n) => sleep(n),
                None => panic!("Not an option")
            }
    }
}