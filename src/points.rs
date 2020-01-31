use rand;
use rand::Rng;
use rand::prelude::ThreadRng;

pub struct Points {
    pub position_x: i8,
    pub position_y: i8,
    rand_gen: ThreadRng,
    size: i8
}

impl Points {
    pub fn new(board_size: i8) -> Points {
        let mut gen = rand::thread_rng();
        Points {
            rand_gen: gen,
            position_x: gen.gen_range(0, board_size - 1),
            position_y: gen.gen_range(0, board_size - 1),
            size: board_size
        }
    }

    pub fn next(&mut self){
        self.position_x = self.rand_gen.gen_range(0, self.size - 1);
        self.position_y = self.rand_gen.gen_range(0, self.size - 1);
    }

    pub fn collision(&mut self, snake_x: i8, snake_y: i8) -> bool{
        snake_x == self.position_x && snake_y == self.position_y
    }
}

use super::renderable::Renderable;
use super::colors;
use super::config;
use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;

impl Renderable for Points {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        let point_x = self.position_x;
        let point_y = self.position_y;

        gl.draw(args.viewport(), |c, gl| {

            let square = rectangle::square(0.0, 0.0, config::STEP);

            let point_trans = c
                    .transform
                    .trans(point_x as f64 * config::STEP, point_y as f64 * config::STEP);
                
            rectangle(colors::BLUE, square, point_trans, gl);
        });
    }
}