use super::{snake::Snake, HEIGHT, WIDTH};

pub trait Drawable {
    fn draw(&self) -> String;
}

impl Drawable for Snake {
    fn draw(&self) -> String {
        let mut grid = [[Cell::Empty; WIDTH]; HEIGHT];

        for (i, [x, y]) in self.body.iter().rev().enumerate() {
            grid[*y][*x] = if i == 0 { Cell::Orange } else { Cell::Green };
        }

        grid[self.apple[1]][self.apple[0]] = Cell::Red;

        grid.draw()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Red,
    Green,
    Orange,
}

impl Drawable for Cell {
    fn draw(&self) -> String {
        match self {
            Cell::Empty => "\x1b[30;1m██\x1b[0m",
            Cell::Red => "\x1b[31m██\x1b[0m",
            Cell::Green => "\x1b[32m██\x1b[0m",
            Cell::Orange => "\x1b[31;1m██\x1b[0m",
        }
        .to_owned()
    }
}

// I'll change this if the width is different than the height.
impl<D> Drawable for [D; WIDTH]
where
    D: Drawable + Copy,
{
    fn draw(&self) -> String {
        self.map(|d| d.draw()).join("")
    }
}

impl<D> Drawable for [D; HEIGHT]
where
    D: Drawable + Copy,
{
    fn draw(&self) -> String {
        self.map(|d| d.draw()).join("\r\n")
    }
}

#[cfg(demo)]
fn demo() {
    let d: Drawable = Cell::Empty;
    d.draw();
    [d; 100].draw();
    [[d; 100]; 100].draw();
    // etc...
}
