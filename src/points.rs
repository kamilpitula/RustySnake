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

    pub fn is_eaten(&mut self, snake_x: i8, snake_y: i8) -> bool{
        snake_x == self.position_x && snake_y == self.position_y
    }
}