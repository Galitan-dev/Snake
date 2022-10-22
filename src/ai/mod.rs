use crate::game::snake::{Snake, Direction};

pub struct AI {
}

impl AI {
    pub fn new() -> Self {
        Self { }
    }

    pub fn get_direction(&self, snake: &Snake) -> Direction {
        let apple = snake.get_apple();
        let head = snake.get_head();

        let dist_x: isize = apple[0] as isize - head[0] as isize;
        let dist_y: isize = apple[1] as isize - head[1] as isize;

        if dist_x < 0 {
            Direction::Left
        } else if dist_x > 0{
            Direction::Right
        } else if dist_y < 0 {
            Direction::Up
        } else {
            Direction::Down
        }
        
    }
}