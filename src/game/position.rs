use bracket_lib::{random::RandomNumberGenerator, terminal::VirtualKeyCode};

use crate::{COLUMNS, ROWS};

pub type UInt = u32;
pub type IInt = i32;

#[derive(Clone, Copy, PartialEq)]
pub struct Position(pub UInt, pub UInt);

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Position {
    pub fn null() -> Self {
        Self(0, 0)
    }

    pub fn random(rng: &mut RandomNumberGenerator) -> Self {
        Self(rng.range(0, COLUMNS), rng.range(0, ROWS))
    }

    pub fn apply(&self, direction: Direction) -> Option<Self> {
        let (diff_x, diff_y) = match direction {
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };

        let new_x = self.0 as IInt + diff_x;
        let new_y = self.1 as IInt + diff_y;

        if new_x < 0 || new_y < 0 || new_x as UInt >= COLUMNS || new_y as UInt >= ROWS {
            None
        } else {
            Some(Self(new_x as UInt, new_y as UInt))
        }
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Self::Right,
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
            Direction::Down => Self::Up,
        }
    }
}

impl TryFrom<Option<VirtualKeyCode>> for Direction {
    type Error = ();

    fn try_from(key: Option<VirtualKeyCode>) -> Result<Self, Self::Error> {
        match key {
            Some(VirtualKeyCode::Left) | Some(VirtualKeyCode::Q) => Ok(Self::Left),
            Some(VirtualKeyCode::Up) | Some(VirtualKeyCode::Z) => Ok(Self::Up),
            Some(VirtualKeyCode::Right) | Some(VirtualKeyCode::D) => Ok(Self::Right),
            Some(VirtualKeyCode::Down) | Some(VirtualKeyCode::S) => Ok(Self::Down),
            _ => Err(()),
        }
    }
}
