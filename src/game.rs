use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use opengl_graphics::{GlGraphics, OpenGL};
use std::time::{Duration, SystemTime};
use std::thread::sleep;

const SEGMENT_SIZE: f64 = 50.0;

//To Snake module:
#[derive(PartialEq)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right
}
//

pub struct Game {
    gl: GlGraphics,
    rotation: f64, //To Delete
    position_x: f64, //To Snake Module
    position_y: f64, //To Snake Module
    direction: SnakeDirection //To Snake Module
}

impl Game{
    pub fn new(opengl_version: OpenGL) -> Game {
        Game {
            gl: GlGraphics::new(opengl_version),
            rotation: 0.0,
            position_x: 0.0,
            position_y: 0.0,
            direction: SnakeDirection::Left
        }
    }

    pub fn render(&mut self, args: &RenderArgs){
            use graphics::*;

            const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, SEGMENT_SIZE);
            let rotation = self.rotation;
            let position_x = self.position_x;
            let position_y = self.position_y;

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

            match self.direction {
                SnakeDirection::Up => self.position_y -= 25.0,
                SnakeDirection::Down => self.position_y += 25.0,
                SnakeDirection::Left => self.position_x -= 25.0,
                SnakeDirection::Right => self.position_x += 25.0
            }

            let difference = start.duration_since(start)
                                    .expect("difference error");
                                    
            let sum_with_frameduration = difference.checked_add(Duration::from_millis(200));

            match sum_with_frameduration {
                Some(n) => sleep(n),
                None => print!("Error")
            }
    }

    pub fn key_press(&mut self, args: &Button){
            const STEP: f64 = 1.5;
            //To Snake Module:
            match *args {
                Keyboard(Key::W) => self.go_up(),
                Keyboard(Key::S) => self.go_down(),
                Keyboard(Key::A) => self.go_left(),
                Keyboard(Key::D) => self.go_right(),
                _ => print!("Rest")
            }
    }
//To Snake Module
    fn go_up(&mut self) {
        if self.direction != SnakeDirection::Down{
            self.direction = SnakeDirection::Up;
        }
    }

    fn go_down(&mut self) {
        if self.direction != SnakeDirection::Up{
            self.direction = SnakeDirection::Down;
        }
    }

    fn go_left(&mut self) {
        if self.direction != SnakeDirection::Right{
            self.direction = SnakeDirection::Left;
        }
    }

    fn go_right(&mut self) {
        if self.direction != SnakeDirection::Left{
            self.direction = SnakeDirection::Right;
        }
    }
}