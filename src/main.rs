extern crate crossterm;

mod game;
mod terminal;

use game::{draw::Drawable, snake::Snake};
use std::{thread, time::Duration};
use terminal::RawTerminal;

const TPS: u64 = 20;

fn main() {
    let terminal = RawTerminal::new();
    let mut snake = Snake::new();

    while let Some(key) = terminal.key() {
        snake.set_direction(key);
        if !snake.update() {
            break;
        }

        terminal.clear();
        terminal.println(snake.draw());

        thread::sleep(Duration::from_millis(1000 / TPS));
    }
}
