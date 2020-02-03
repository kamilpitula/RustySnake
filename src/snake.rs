#[derive(PartialEq)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right
}

pub struct Snake {
    direction: SnakeDirection,
    pub tail: Vec<(i8, i8)>
}

impl Snake{
    pub fn new() -> Snake {
        let mut snake = Snake {
            direction: SnakeDirection::Right,
            tail: Vec::new()
        };
        snake.tail.push((0, 0));
        return snake;
    }

    pub fn update_position<F>(&mut self, range_checker: F)
        where F: Fn(i8) -> i8 {

            for i in (1..self.tail.len()).rev() {
                let (x, y) = self.tail[i - 1];
                self.tail[i] = (x, y);
            }

            let (position_x, position_y) = self.tail[0];

            match self.direction {
                SnakeDirection::Up => self.tail[0] = (position_x, range_checker(position_y - 1)),
                SnakeDirection::Down => self.tail[0] = (position_x, range_checker(position_y + 1)),
                SnakeDirection::Left => self.tail[0] = (range_checker(position_x - 1), position_y),
                SnakeDirection::Right => self.tail[0] = (range_checker(position_x + 1), position_y)
            };
    }

    pub fn go_up(&mut self) {
        if self.direction != SnakeDirection::Down{
            self.direction = SnakeDirection::Up;
        }
    }

    pub fn go_down(&mut self) {
        if self.direction != SnakeDirection::Up{
            self.direction = SnakeDirection::Down;
        }
    }

    pub fn go_left(&mut self) {
        if self.direction != SnakeDirection::Right{
            self.direction = SnakeDirection::Left;
        }
    }

    pub fn go_right(&mut self) {
        if self.direction != SnakeDirection::Left{
            self.direction = SnakeDirection::Right;
        }
    }

    pub fn add_tail_element(&mut self){
        let (x, y) = self.tail[0];
        self.tail.push((x, y));
    }

    pub fn self_collision(&mut self) -> bool {
        let (x, y) = self.tail[0];
        let filtered:Vec<&(i8,i8)> = self.tail.iter().filter(|(a,b)| x==*a && y==*b).collect();
        filtered.len() > 1
    }
}

use super::renderable::Renderable;
use super::colors;
use super::config;
use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::Context;

impl Renderable for Snake {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        
        let tail = &self.tail;

        let square = rectangle::square(0.0, 0.0, config::STEP);

        for (x, y) in tail {
            let transform = ctx
            .transform
            .trans(*x as f64 * config::STEP, *y as f64 * config::STEP);

            rectangle(colors::RED, square, transform, gl);
         }
    }
}