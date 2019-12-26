#[derive(PartialEq)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right
}

pub struct Snake {
    pub position_x: i8,
    pub position_y: i8,
    direction: SnakeDirection
}

impl Snake{
    pub fn new() -> Snake {
        Snake {
            position_x: 0,
            position_y: 0,
            direction: SnakeDirection::Right
        }
    }

    pub fn update_position<F>(&mut self, range_checker: F)
        where F: Fn(i8) -> i8 {
        match self.direction {
            SnakeDirection::Up => self.position_y = range_checker(self.position_y - 1),
            SnakeDirection::Down => self.position_y = range_checker(self.position_y + 1),
            SnakeDirection::Left => self.position_x = range_checker(self.position_x - 1),
            SnakeDirection::Right => self.position_x = range_checker(self.position_x + 1)
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
}