use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use opengl_graphics::{GlGraphics, OpenGL};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use super::snake::Snake;

const SEGMENT_SIZE: f64 = 50.0;

pub struct Game {
    gl: GlGraphics,
    snake: Snake
}

impl Game{
    pub fn new(opengl_version: OpenGL) -> Game {
        Game {
            gl: GlGraphics::new(opengl_version),
            snake: Snake::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs){
            use graphics::*;

            const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, SEGMENT_SIZE);
            let position_x = self.snake.position_x;
            let position_y = self.snake.position_y;

            let(x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            self.gl.draw(args.viewport(), |c, gl| {
                clear(GRAY, gl);

                let transform = c
                    .transform
                    .trans(x + position_x, y + position_y)
                    .trans(-25.0, -25.0);

                rectangle(RED, square, transform, gl);
            });
    }

    pub fn update(&mut self, args: &UpdateArgs){
            let start = SystemTime::now();

            self.snake.update_position();

            self.fill_empty_frame_time(&start);
    }

    pub fn key_press(&mut self, args: &Button){
            match *args {
                Keyboard(Key::W) => self.snake.go_up(),
                Keyboard(Key::S) => self.snake.go_down(),
                Keyboard(Key::A) => self.snake.go_left(),
                Keyboard(Key::D) => self.snake.go_right(),
                _ => print!("Rest")
            }
    }

    fn fill_empty_frame_time(&mut self, frame_start_time: &SystemTime){
            let difference = frame_start_time.duration_since(*frame_start_time)
            .expect("difference error");
        
            let sum_with_frameduration = difference.checked_add(Duration::from_millis(200));

            match sum_with_frameduration {
                Some(n) => sleep(n),
                None => print!("Error")
            }
    }
}