mod utils;

use crate::game::snake::{Snake, Direction};

use self::utils::dist;

pub struct AI {
}

impl AI {
    pub fn new() -> Self {
        Self { }
    }

    pub fn get_direction(&self, snake: &Snake) -> Direction {
        let apple = snake.get_apple();
        let head = snake.get_head();

        let mut dirs = Direction::list();
        dirs.sort_by(|a, b| 
            dist(a.apply(head).unwrap_or(head), apple)
            .cmp(&dist(b.apply(head).unwrap_or(head), apple)));

        for dir in dirs {
            if let Ok(new_head) = dir.apply(head) {
                if !snake.get_body().contains(&new_head) {
                    return dir
                }
            }
        }
        
        snake.get_direction()
    }
}