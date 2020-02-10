use rand;
use rand::Rng;
use rand::prelude::ThreadRng;

pub struct Point {
    pub position_x: i16,
    pub position_y: i16,
    pub is_bonus: bool,
    rand_gen: ThreadRng,
    bonus_chance_percent: i16,
    size: i16
}

impl Point {
    pub fn new(board_size: i16, bonus_chance_percent: i16) -> Point {
        let mut gen = rand::thread_rng();
        Point {
            rand_gen: gen,
            position_x: gen.gen_range(0, board_size - 1),
            position_y: gen.gen_range(0, board_size - 1),
            size: board_size,
            bonus_chance_percent: bonus_chance_percent,
            is_bonus: false
        }
    }

    pub fn next(&mut self){
        self.process_bonus();
        
        self.position_x = self.rand_gen.gen_range(0, self.size - 1);
        self.position_y = self.rand_gen.gen_range(0, self.size - 1);
    }

    pub fn collision(&mut self, snake_x: i16, snake_y: i16) -> bool{
        snake_x == self.position_x && snake_y == self.position_y
    }

    pub fn process_bonus(&mut self){
        let rand = self.rand_gen.gen_range(1, 100);

        if rand <= self.bonus_chance_percent {
            self.is_bonus = true;
        }
        else {
            self.is_bonus = false;
        }
    }
}

use super::renderable::Renderable;
use super::colors;
use super::config;
use opengl_graphics::GlGraphics;
use graphics::Context;

impl Renderable for Point {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        let mut color = colors::BLUE;

        let point_x = self.position_x;
        let point_y = self.position_y;

        let square = rectangle::square(0.0, 0.0, config::STEP as f64);

        let point_trans = ctx
                .transform
                .trans(point_x as f64 * config::STEP as f64, point_y as f64 * config::STEP as f64);
        
        if self.is_bonus {
            color = colors::GREEN;
        }
                
        rectangle(color, square, point_trans, gl);
    }
}