#[derive(PartialEq)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right
}

pub struct Snake {
    pub position_x: f64,
    pub position_y: f64,
    direction: SnakeDirection
}

impl Snake{
    pub fn new() -> Snake {
        Snake {
            position_x: 0.0,
            position_y: 0.0,
            direction: SnakeDirection::Left
        }
    }

    pub fn update_position(&mut self) {
        match self.direction {
            SnakeDirection::Up => self.position_y -= 25.0,
            SnakeDirection::Down => self.position_y += 25.0,
            SnakeDirection::Left => self.position_x -= 25.0,
            SnakeDirection::Right => self.position_x += 25.0
        }

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