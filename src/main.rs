extern crate crossterm;

mod game;
mod terminal;
mod ai;

use game::{draw::Drawable, snake::Snake};
use std::{thread, time::Duration, env};
use terminal::RawTerminal;
use ai::AI;

const TPS: u64 = 20;

fn main() {
    let ai_arg = env::var("AI").unwrap_or("0".to_owned());
    let use_ai = ai_arg == "1";

    let terminal = RawTerminal::new();
    let mut snake = Snake::new();
    let ai = AI::new();

    while let Some(key) = terminal.key() {
        
        if use_ai {
            snake.set_direction( ai.get_direction(&snake))
        } else {
            snake.set_direction(key);
        }
        
        if !snake.update() {
            break;
        }

        terminal.clear();
        terminal.println(snake.draw());

        thread::sleep(Duration::from_millis(1000 / TPS));
    }
}
