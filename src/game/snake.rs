use rand::{prelude::ThreadRng, thread_rng, Rng};
use crate::terminal::key::Key;
use super::{HEIGHT, WIDTH};

pub struct Snake {
    pub(super) apple: [usize; 2],
    pub(super) body: Vec<[usize; 2]>,
    direction: Direction,
    rng: ThreadRng,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl TryFrom<Key> for Direction {
    type Error = ();

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        match key {
            Key::Up => Ok(Direction::Up),
            Key::Down => Ok(Direction::Down),
            Key::Left => Ok(Direction::Left),
            Key::Right => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn list() -> [Self; 4] {
        [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right
        ]
    }

    pub fn apply(&self, pos: [usize; 2]) -> Result<[usize; 2], ()> {
        let [sx, sy] = match self {
            Direction::Up => [0, -1],
            Direction::Left => [-1, 0],
            Direction::Down => [0, 1],
            Direction::Right => [1, 0],
        };

        let x = pos[0] as isize + sx;
        let y = pos[1] as isize + sy;

        if x < 0 || y < 0 || x >= WIDTH as _ || y >= HEIGHT as _ {
            Err(())
        } else {
            Ok([x as usize, y as usize])
        }
    }

    pub fn is_opposite(&self, other: &Direction) -> bool {
        match [self, other] {
            [Direction::Up | Direction::Down, Direction::Down | Direction::Up]
            | [Direction::Left | Direction::Right, Direction::Right | Direction::Left] => true,
            _ => false,
        }
    }
}

impl Snake {
    pub fn new() -> Self {
        Self {
            apple: [WIDTH / 2, HEIGHT / 2],
            body: vec![[3, 3], [4, 3], [5, 3]],
            direction: Direction::Right,
            rng: thread_rng(),
        }
    }

    pub fn update(&mut self) -> bool {
        let head = self.get_head();

        if let Ok(new_head) = self.direction.apply(*self.body.last().unwrap()) {
            if self.body.contains(&new_head) {
                return false;
            }
            self.body.push(new_head);
        } else {
            return false;
        }

        if head == self.apple {
            while self.body.contains(&self.apple) {
                self.apple = [self.rng.gen_range(0..WIDTH), self.rng.gen_range(1..HEIGHT)];
            }
        } else {
            self.body.remove(0);
        }

        true
    }

    pub fn get_head(&self) -> [usize; 2] {
        *self.body.last().unwrap()
    }

    pub fn get_body(&self) -> &Vec<[usize; 2]> {
        &self.body
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_apple(&self) -> [usize; 2] {
        self.apple
    }

    pub fn set_direction<D>(&mut self, dir: D)
    where
        D: TryInto<Direction>,
    {
        if let Ok(dir) = dir.try_into() {
            if !self.direction.is_opposite(&dir) {
                self.direction = dir;
            }
        }
    }
}
