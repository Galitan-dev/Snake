use super::{Direction, Position};
use crate::palette;
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{BTerm, GameState},
};
use std::{collections::VecDeque, time::Instant};

pub struct State {
    snake: VecDeque<Position>,
    is_first_render: bool,
    last_tick: Instant,
    last_tail: Position,
    direction: Direction,
    next_direction: Direction,
    apple: Position,
    has_apple_moved: bool,
    rng: RandomNumberGenerator,
}

impl State {
    pub fn new() -> Self {
        let mut state = Self {
            snake: VecDeque::from(vec![Position(5, 3), Position(4, 3), Position(3, 3)]),
            is_first_render: true,
            last_tick: Instant::now(),
            last_tail: Position(2, 3),
            direction: Direction::Right,
            next_direction: Direction::Right,
            apple: Position::null(),
            has_apple_moved: true,
            rng: RandomNumberGenerator::new(),
        };

        state.move_apple();

        state
    }

    fn render(&mut self, ctx: &mut BTerm) {
        if self.is_first_render {
            self.first_render(ctx);
            self.is_first_render = false;
        } else {
            ctx.set_bg(self.last_tail.0, self.last_tail.1, palette::color("grey"));

            let head = self.get_snake_head();
            ctx.set_bg(head.0, head.1, palette::color("pink"));

            let last_head = self.snake.get(1).expect("Tiny Snake");
            ctx.set_bg(last_head.0, last_head.1, palette::color("green"));

            if self.has_apple_moved {
                ctx.set_bg(self.apple.0, self.apple.1, palette::color("red"));
            }
        }
    }

    fn first_render(&self, ctx: &mut BTerm) {
        ctx.cls_bg(palette::color("grey"));

        for Position(x, y) in &self.snake {
            ctx.set_bg(*x, *y, palette::color("green"));
        }

        let Position(x, y) = self.get_snake_head();
        ctx.set_bg(*x, *y, palette::color("pink"));

        ctx.set_bg(self.apple.0, self.apple.1, palette::color("red"));
    }

    fn get_snake_head(&self) -> &Position {
        self.snake.front().expect("Empty Snake")
    }

    fn move_apple(&mut self) {
        while {
            self.apple = Position::random(&mut self.rng);
            self.get_snake_head() == &self.apple
        } {}
        self.has_apple_moved = true;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.next_direction = ctx
            .key
            .try_into()
            .and_then(|direction| {
                if direction == self.direction.opposite() {
                    Err(())
                } else {
                    Ok(direction)
                }
            })
            .unwrap_or(self.next_direction);

        self.direction = self.next_direction;
        self.last_tick = Instant::now();

        if self.get_snake_head() == &self.apple {
            self.move_apple();
        } else {
            self.last_tail = self.snake.pop_back().expect("Empty Snake");
        }

        if let Some(new_head) = self.get_snake_head().apply(self.direction) {
            self.snake.push_front(new_head);

            self.render(ctx);
            self.has_apple_moved = false;
        } else {
            ctx.quit();
        }
    }
}
